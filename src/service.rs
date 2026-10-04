//! High-level service layer between the CLI and the raw API client.
//!
//! [`WikiService`] owns a [`WikiClient`] and is where cross-cutting concerns will
//! live: caching, language handling, loading/progress reporting, etc. For now it
//! just exposes the two read operations the reader needs.

use std::path::PathBuf;

use crate::wiki::{self, WikiClient, WikiConfig};

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

    /// Search pages and print the matches.
    pub fn search(&self, query: &str, limit: u8) -> wiki::Result<()> {
        let results = self.client.search(query, limit)?;
        if results.is_empty() {
            eprintln!("no results for {query:?}");
            return Ok(());
        }
        for result in results {
            println!("{}", result.title);
            if let Some(description) = result.description.as_deref().filter(|d| !d.is_empty()) {
                println!("    {description}");
            }
        }
        Ok(())
    }

    /// Fetch a page's latest public revision as wikitext, optionally saving it.
    pub fn article(&self, title: &str, save: Option<PathBuf>, print: bool) -> wiki::Result<()> {
        let page = self.client.get_page(title)?;
        let source = page.source.as_deref().unwrap_or_default();

        eprintln!(
            "{} — revision {} ({})",
            page.title, page.latest.id, page.latest.timestamp
        );

        if let Some(path) = &save {
            std::fs::write(path, source)?;
            eprintln!("saved {} bytes to {}", source.len(), path.display());
        }
        if print || save.is_none() {
            println!("{source}");
        }
        Ok(())
    }
}
