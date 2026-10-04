//! Minimal synchronous client for the MediaWiki REST API.
//!
//! Documentation: <https://www.mediawiki.org/wiki/API:REST_API>
//!
//! Only the read endpoints needed by the reader are implemented: page search and
//! fetching a page's latest public revision as wikitext.

use std::time::Duration;

use percent_encoding::{
    AsciiSet, CONTROLS, NON_ALPHANUMERIC, percent_decode_str, utf8_percent_encode,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Characters percent-encoded when placing a page title in a URL. Space is
/// handled separately (turned into `_`); `?` and `#` are encoded so a title can
/// never be mistaken for a query string or fragment.
const TITLE_ENCODE: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'[')
    .add(b']')
    .add(b'{')
    .add(b'}')
    .add(b'|')
    .add(b'\\')
    .add(b'^')
    .add(b'`');

/// Default `User-Agent`, including contact information as required by the
/// [Wikimedia User-Agent policy](https://foundation.wikimedia.org/wiki/Policy:Wikimedia_Foundation_User-Agent_Policy).
///
/// Override at runtime with [`WikiConfig::from_env`] / the `--user-agent` flag.
pub const DEFAULT_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/gojoGS/web_reader)",
);

/// Everything that can go wrong while talking to the wiki.
#[derive(Debug, Error)]
pub enum WikiError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("could not decode JSON response: {0}")]
    Decode(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid header value: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    #[error("{url} returned HTTP {status}: {body}")]
    Status {
        url: String,
        status: u16,
        body: String,
    },

    #[error("response for {context} did not contain a `{field}` field")]
    MissingField {
        field: &'static str,
        context: String,
    },

    #[error("not an article URL produced by this site: {url}")]
    InvalidArticleUrl { url: String },
}

pub type Result<T> = std::result::Result<T, WikiError>;

/// Connection settings for a single wiki (project + language).
#[derive(Debug, Clone)]
pub struct WikiConfig {
    /// Project host, e.g. `wikipedia` or `wiktionary`.
    pub project: String,
    /// Language code, e.g. `en`, `de`, `fr`.
    pub language: String,
    /// `User-Agent` header sent with every request.
    pub user_agent: String,
    /// Per-request timeout.
    pub timeout: Duration,
}

impl Default for WikiConfig {
    fn default() -> Self {
        Self {
            project: "wikipedia".to_string(),
            language: "en".to_string(),
            user_agent: DEFAULT_USER_AGENT.to_string(),
            timeout: Duration::from_secs(30),
        }
    }
}

impl WikiConfig {
    /// Build a config, honouring the `WEB_READER_USER_AGENT` and
    /// `WEB_READER_LANG` environment variables when set.
    pub fn from_env() -> Self {
        let mut config = Self::default();
        if let Some(ua) = non_empty_env("WEB_READER_USER_AGENT") {
            config.user_agent = ua;
        }
        if let Some(lang) = non_empty_env("WEB_READER_LANG") {
            config.language = lang;
        }
        config
    }

    /// Base URL of the REST API, e.g. `https://en.wikipedia.org/w/rest.php/v1`.
    pub fn rest_base(&self) -> String {
        format!(
            "https://{}.{}.org/w/rest.php/v1",
            self.language, self.project
        )
    }
}

/// Synchronous read-only client for one wiki.
#[derive(Debug, Clone)]
pub struct WikiClient {
    http: reqwest::blocking::Client,
    rest_base: String,
}

impl WikiClient {
    pub fn new(config: WikiConfig) -> Result<Self> {
        let rest_base = config.rest_base();
        let http = reqwest::blocking::Client::builder()
            .user_agent(config.user_agent)
            .timeout(config.timeout)
            .gzip(true)
            .build()?;
        Ok(Self { http, rest_base })
    }

    /// Base URL this client talks to.
    pub fn rest_base(&self) -> &str {
        &self.rest_base
    }

    /// Full-text page search. `limit` is clamped to the API's 1..=100 range.
    pub fn search(&self, query: &str, limit: u8) -> Result<Vec<SearchResult>> {
        let limit = limit.clamp(1, 100);
        let data: SearchResponse = self.get_json(
            "/search/page",
            &[("q", query.to_string()), ("limit", limit.to_string())],
        )?;
        Ok(data.pages)
    }

    /// Fetch a page's latest public revision, including its wikitext `source`.
    pub fn get_page(&self, title: &str) -> Result<Page> {
        let encoded = utf8_percent_encode(title, NON_ALPHANUMERIC).to_string();
        self.get_json(&format!("/page/{encoded}"), &[])
    }

    /// Fetch just the latest wikitext of a page.
    pub fn get_wikitext(&self, title: &str) -> Result<String> {
        let page = self.get_page(title)?;
        page.source.ok_or_else(|| WikiError::MissingField {
            field: "source",
            context: format!("page '{}'", page.title),
        })
    }

    fn get_json<T: DeserializeOwned>(&self, path: &str, query: &[(&str, String)]) -> Result<T> {
        let url = format!("{}{}", self.rest_base, path);
        let response = self.http.get(&url).query(query).send()?;
        let status = response.status();
        let body = response.text()?;
        if !status.is_success() {
            return Err(WikiError::Status {
                url,
                status: status.as_u16(),
                body: truncate(&body, 500),
            });
        }
        Ok(serde_json::from_str(&body)?)
    }
}

/// Top-level object returned by `GET /search/page`.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub pages: Vec<SearchResult>,
}

/// One page match from the search endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: u64,
    /// Title in URL-friendly form.
    pub key: String,
    /// Title in reading-friendly form.
    pub title: String,
    /// Highlighted content excerpt (search endpoint).
    #[serde(default)]
    pub excerpt: String,
    /// Short description sourced from Wikidata, if any.
    #[serde(default)]
    pub description: Option<String>,
    /// Title of the redirect this result came from, if any.
    #[serde(default)]
    pub matched_title: Option<String>,
    /// Lead image thumbnail, if any.
    #[serde(default)]
    pub thumbnail: Option<Thumbnail>,
}

impl SearchResult {
    /// Browser-ready article URL, built from the result's URL-friendly `key`.
    ///
    /// This is exactly the form [`title_from_url`] accepts, so the output of
    /// `search` can be fed straight back into `article`.
    pub fn url(&self, site_base: &str) -> String {
        format!("{site_base}{}", encode_title(&self.key))
    }
}

/// Percent-encode a page title for use in a wiki URL (spaces become `_`).
pub fn encode_title(title: &str) -> String {
    utf8_percent_encode(&title.replace(' ', "_"), TITLE_ENCODE).to_string()
}

/// Recover a page title from a URL that [`encode_title`] / [`SearchResult::url`]
/// produced. Returns `None` if `url` is not under `site_base`.
///
/// Assumes the URL came from this project, so no host validation beyond the
/// `site_base` prefix is performed.
pub fn title_from_url(site_base: &str, url: &str) -> Option<String> {
    let rest = url.trim().strip_prefix(site_base)?;
    let rest = rest.split(['?', '#']).next().unwrap_or(rest);
    if rest.is_empty() {
        return None;
    }
    let decoded = percent_decode_str(rest).decode_utf8_lossy();
    Some(decoded.replace('_', " "))
}

/// Reduced-size lead image returned with a search result.
#[derive(Debug, Serialize, Deserialize)]
pub struct Thumbnail {
    #[serde(default)]
    pub mimetype: Option<String>,
    #[serde(default)]
    pub width: Option<u32>,
    #[serde(default)]
    pub height: Option<u32>,
    #[serde(default)]
    pub url: Option<String>,
}

/// Page object returned by `GET /page/{title}`.
#[derive(Debug, Deserialize)]
pub struct Page {
    pub id: u64,
    /// Title in URL-friendly form.
    pub key: String,
    /// Title in reading-friendly form.
    pub title: String,
    /// Information about the latest public revision.
    pub latest: Revision,
    /// Content model, typically `wikitext`.
    #[serde(default)]
    pub content_model: Option<String>,
    /// License the content is published under.
    #[serde(default)]
    pub license: Option<License>,
    /// Latest page content in the `content_model` format (wikitext).
    #[serde(default)]
    pub source: Option<String>,
    /// REST route that returns the page content as HTML.
    #[serde(default)]
    pub html_url: Option<String>,
}

/// Latest-revision metadata.
#[derive(Debug, Deserialize)]
pub struct Revision {
    /// Revision ID — the latest public revision of the page.
    pub id: u64,
    /// Revision timestamp in ISO 8601 format.
    pub timestamp: String,
}

/// License metadata for a page.
#[derive(Debug, Clone, serde::Serialize, Deserialize)]
pub struct License {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
}

fn non_empty_env(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn truncate(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let mut out: String = text.chars().take(max_chars).collect();
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = "https://en.wikipedia.org/wiki/";

    fn result(key: &str) -> SearchResult {
        SearchResult {
            id: 1,
            key: key.to_string(),
            title: key.replace('_', " "),
            excerpt: String::new(),
            description: None,
            matched_title: None,
            thumbnail: None,
        }
    }

    #[test]
    fn url_round_trips_through_title_from_url() {
        for title in [
            "Rust (programming language)",
            "München",
            "C++",
            "Earth",
            "A/B subpage",
            "Who?",
        ] {
            let url = format!("{BASE}{}", encode_title(title));
            assert_eq!(
                title_from_url(BASE, &url).as_deref(),
                Some(title),
                "title={title:?} url={url}"
            );
        }
    }

    #[test]
    fn search_result_url_is_browser_ready() {
        let url = result("Rust_(programming_language)").url(BASE);
        assert_eq!(
            url,
            "https://en.wikipedia.org/wiki/Rust_(programming_language)"
        );
    }

    #[test]
    fn non_ascii_and_punctuation_are_encoded() {
        assert_eq!(encode_title("München"), "M%C3%BCnchen");
        assert_eq!(encode_title("Who?"), "Who%3F");
    }

    #[test]
    fn title_from_url_rejects_foreign_or_non_urls() {
        assert_eq!(
            title_from_url(BASE, "https://de.wikipedia.org/wiki/Erde"),
            None
        );
        assert_eq!(title_from_url(BASE, "Rust"), None);
        assert_eq!(title_from_url(BASE, "https://en.wikipedia.org/wiki/"), None);
    }

    #[test]
    fn title_from_url_ignores_query_and_fragment() {
        assert_eq!(
            title_from_url(BASE, "https://en.wikipedia.org/wiki/Earth?oldid=1#History").as_deref(),
            Some("Earth")
        );
    }
}
