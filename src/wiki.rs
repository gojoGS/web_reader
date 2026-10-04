//! Minimal synchronous client for the MediaWiki REST API.
//!
//! Documentation: <https://www.mediawiki.org/wiki/API:REST_API>
//!
//! Only the read endpoints needed by the reader are implemented: page search and
//! fetching a page's latest public revision as wikitext.

use std::time::Duration;

use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use thiserror::Error;

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
#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub pages: Vec<SearchResult>,
}

/// One page match from the search endpoint.
#[derive(Debug, Deserialize)]
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

/// Reduced-size lead image returned with a search result.
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
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
