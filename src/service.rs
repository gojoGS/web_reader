//! High-level service layer between the CLI and the raw API client.
//!
//! [`WikiService`] owns a [`WikiClient`] and is where cross-cutting concerns will
//! live: caching, language handling, loading/progress reporting, etc. It returns
//! plain data; presenting or persisting it is the caller's job.

use crate::wiki::{self, SearchResult, WikiClient, WikiConfig};

/// A page's latest public revision, ready to be converted or displayed.
#[derive(Debug, Clone)]
pub struct Article {
    /// Title in reading-friendly form.
    pub title: String,
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
}

impl WikiService {
    /// Build a service from the given configuration.
    pub fn new(config: WikiConfig) -> wiki::Result<Self> {
        Ok(Self {
            client: WikiClient::new(config)?,
        })
    }

    /// Full-text page search.
    pub fn search(&self, query: &str, limit: u8) -> wiki::Result<Vec<SearchResult>> {
        self.client.search(query, limit)
    }

    /// Fetch a page's latest public revision as wikitext.
    pub fn article(&self, title: &str) -> wiki::Result<Article> {
        let page = self.client.get_page(title)?;
        let wikitext = page.source.ok_or_else(|| wiki::WikiError::MissingField {
            field: "source",
            context: format!("page '{}'", page.title),
        })?;
        Ok(Article {
            title: page.title,
            revision_id: page.latest.id,
            revision_timestamp: page.latest.timestamp,
            wikitext,
            license: page.license,
        })
    }
}
