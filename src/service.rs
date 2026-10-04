//! High-level service layer between the CLI and the raw API client.
//!
//! [`WikiService`] owns a [`WikiClient`] and an [`ArticleCache`]. It returns plain
//! data; presenting or persisting it is the caller's job.
//!
//! ## Language
//!
//! Page and revision IDs are local to a single language wiki — `Earth` on
//! `en.wikipedia.org` (page 9228) and `Erde` on `de.wikipedia.org` (page 1320)
//! share neither ID. Language versions are only linked by topic (Wikidata
//! sitelinks), exposed by `GET /page/{title}/links/language` as translated
//! titles, not by a shared ID. The cache is therefore partitioned per language.

use chrono::{Duration, Utc};

use crate::cache::{ArticleCache, ArticleMeta, DEFAULT_TTL_SECS, LatestEntry};
use crate::wiki::{self, SearchResult, WikiClient, WikiConfig};

/// A page's latest public revision, ready to be converted or displayed.
#[derive(Debug, Clone)]
pub struct Article {
    /// Title in reading-friendly form.
    pub title: String,
    /// Language code of the wiki this article came from.
    pub language: String,
    /// Page ID (local to the language wiki).
    pub page_id: u64,
    /// ID of the latest public revision.
    pub revision_id: u64,
    /// Timestamp of the latest public revision (ISO 8601).
    pub revision_timestamp: String,
    /// Latest revision content as wikitext.
    pub wikitext: String,
    /// License the content is published under.
    pub license: Option<wiki::License>,
}

/// Application-facing entry point for reading a wiki.
#[derive(Debug, Clone)]
pub struct WikiService {
    client: WikiClient,
    language: String,
    site_base: String,
    cache: ArticleCache,
}

impl WikiService {
    /// Build a service from the given configuration.
    pub fn new(config: WikiConfig) -> wiki::Result<Self> {
        let language = config.language.clone();
        let site_base = format!("https://{}.{}.org/wiki/", config.language, config.project);
        let client = WikiClient::new(config)?;
        let cache = ArticleCache::from_env(&language, ttl_from_env());
        Ok(Self {
            client,
            language,
            site_base,
            cache,
        })
    }

    /// Language code this service reads from.
    pub fn language(&self) -> &str {
        &self.language
    }

    /// Base URL used to resolve internal wiki links, e.g.
    /// `https://en.wikipedia.org/wiki/`.
    pub fn site_base(&self) -> &str {
        &self.site_base
    }

    /// Full-text page search.
    pub fn search(&self, query: &str, limit: u8) -> wiki::Result<Vec<SearchResult>> {
        self.client.search(query, limit)
    }

    /// Fetch a page's latest public revision as wikitext, using the on-disk
    /// cache when a fresh entry exists for this language.
    pub fn article(&self, title: &str) -> wiki::Result<Article> {
        let cached = self.cache.load(title);

        if let Some(entry) = &cached {
            if entry.is_fresh(Utc::now()) {
                let article = Article {
                    title: entry.meta.title.clone(),
                    language: self.language.clone(),
                    page_id: entry.meta.page_id,
                    revision_id: entry.meta.revision_id,
                    revision_timestamp: entry.meta.revision_timestamp.clone(),
                    wikitext: entry.wikitext.clone(),
                    license: entry.meta.license.clone(),
                };
                self.record_lookup(&article, true);
                return Ok(article);
            }
        }

        let page = self.client.get_page(title)?;
        let wikitext = page.source.ok_or_else(|| wiki::WikiError::MissingField {
            field: "source",
            context: format!("page '{}'", page.title),
        })?;

        let article = Article {
            title: page.title,
            language: self.language.clone(),
            page_id: page.id,
            revision_id: page.latest.id,
            revision_timestamp: page.latest.timestamp,
            wikitext,
            license: page.license,
        };

        let now = Utc::now();
        let meta = ArticleMeta {
            schema: ArticleMeta::SCHEMA,
            title: article.title.clone(),
            language: article.language.clone(),
            page_id: article.page_id,
            revision_id: article.revision_id,
            revision_timestamp: article.revision_timestamp.clone(),
            license: article.license.clone(),
            cached_at: now,
            expires_at: now + self.cache.ttl(),
        };

        // Cache writes are best-effort: a read must not fail because the cache
        // directory is unwritable.
        match &cached {
            Some(entry)
                if entry.meta.revision_id == meta.revision_id && !entry.wikitext.is_empty() =>
            {
                // Same revision: keep the wikitext, refresh only the TTL.
                let _ = self.cache.update_meta(&meta);
            }
            _ => {
                // New revision (or a missing payload): rewrite both files.
                let _ = self.cache.store(&meta, &article.wikitext);
            }
        }

        self.record_lookup(&article, false);
        Ok(article)
    }

    /// The `n` most recent successful lookups, newest first.
    pub fn recent_lookups(&self, n: usize) -> Vec<LatestEntry> {
        self.cache.recent(n)
    }

    /// Fetch a page and render its wikitext as Markdown.
    pub fn markdown(&self, title: &str) -> wiki::Result<String> {
        let article = self.article(title)?;
        Ok(crate::markdown::to_markdown(
            &article.wikitext,
            &self.site_base,
        ))
    }

    fn record_lookup(&self, article: &Article, from_cache: bool) {
        let entry = LatestEntry {
            language: article.language.clone(),
            title: article.title.clone(),
            page_id: article.page_id,
            revision_id: article.revision_id,
            revision_timestamp: article.revision_timestamp.clone(),
            looked_up_at: Utc::now(),
            from_cache,
        };
        // Best-effort, like the rest of the cache.
        let _ = self.cache.record_latest(entry);
    }
}

/// TTL for cached articles, overridable with `WEB_READER_CACHE_TTL_SECS`.
fn ttl_from_env() -> Duration {
    let secs = std::env::var("WEB_READER_CACHE_TTL_SECS")
        .ok()
        .and_then(|raw| raw.trim().parse::<i64>().ok())
        .filter(|secs| *secs > 0)
        .unwrap_or(DEFAULT_TTL_SECS);
    Duration::seconds(secs)
}
