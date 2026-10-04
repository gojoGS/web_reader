//! On-disk cache for fetched articles.
//!
//! Layout (under the platform cache dir, or `WEB_READER_CACHE_DIR`):
//!
//! ```text
//! <base>/articles/<language>/<sha256(title)>/meta.json
//! <base>/articles/<language>/<sha256(title)>/wikitext.txt
//! <base>/latest.json
//! ```
//!
//! Wikitext is stored separately from its metadata so that refreshing the TTL of
//! an unchanged revision only rewrites the small `meta.json`. The language is
//! part of the path because page and revision IDs are local to one wiki: the same
//! topic has different IDs in `en.wikipedia.org` and `de.wikipedia.org`.
//!
//! `latest.json` is a cross-language index of successfully looked-up articles,
//! ordered newest-first so the last `n` lookups are just the first `n` entries.

use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::wiki::License;

/// Default time-to-live for a cached article.
pub const DEFAULT_TTL_SECS: i64 = 24 * 60 * 60;

/// Metadata stored alongside a cached article's wikitext.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleMeta {
    /// Cache schema version; bump to invalidate old entries.
    pub schema: u32,
    /// Title in reading-friendly form.
    pub title: String,
    /// Language code this entry belongs to.
    pub language: String,
    /// Page ID (local to the language wiki).
    pub page_id: u64,
    /// Latest public revision ID.
    pub revision_id: u64,
    /// Latest public revision timestamp (ISO 8601).
    pub revision_timestamp: String,
    /// License the content is published under.
    #[serde(default)]
    pub license: Option<License>,
    /// When this entry was last written.
    pub cached_at: DateTime<Utc>,
    /// When this entry stops being usable.
    pub expires_at: DateTime<Utc>,
}

impl ArticleMeta {
    /// Current cache schema version.
    pub const SCHEMA: u32 = 1;

    /// Whether this entry is still within its TTL.
    pub fn is_fresh(&self, now: DateTime<Utc>) -> bool {
        self.schema == Self::SCHEMA && self.expires_at > now
    }
}

/// A cached article: its metadata plus the wikitext payload.
#[derive(Debug, Clone)]
pub struct CachedArticle {
    pub meta: ArticleMeta,
    /// Empty when the payload file is missing or unreadable.
    pub wikitext: String,
}

impl CachedArticle {
    /// Fresh entries have both a valid TTL and a non-empty payload.
    pub fn is_fresh(&self, now: DateTime<Utc>) -> bool {
        self.meta.is_fresh(now) && !self.wikitext.is_empty()
    }
}

/// One entry in the cross-language `latest.json` lookup index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestEntry {
    /// Language code the article belongs to.
    pub language: String,
    /// Title in reading-friendly form.
    pub title: String,
    /// Page ID (local to the language wiki).
    pub page_id: u64,
    /// Latest public revision ID.
    pub revision_id: u64,
    /// Latest public revision timestamp (ISO 8601).
    pub revision_timestamp: String,
    /// When the lookup happened.
    pub looked_up_at: DateTime<Utc>,
    /// Whether the lookup was served from the cache.
    pub from_cache: bool,
}

/// The `latest.json` document. `entries` is ordered newest-first.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestIndex {
    /// Index schema version; bump to invalidate old files.
    pub schema: u32,
    /// Successfully looked-up articles, most recent first.
    pub entries: Vec<LatestEntry>,
}

impl LatestIndex {
    /// Current index schema version.
    pub const SCHEMA: u32 = 1;
}

impl Default for LatestIndex {
    fn default() -> Self {
        Self {
            schema: Self::SCHEMA,
            entries: Vec::new(),
        }
    }
}

/// Filesystem-backed article cache for one language wiki, sharing a common
/// `<base>` directory with other languages for the `latest.json` index.
#[derive(Debug, Clone)]
pub struct ArticleCache {
    base: PathBuf,
    root: PathBuf,
    ttl: Duration,
}

impl ArticleCache {
    /// Resolve the cache directory from `WEB_READER_CACHE_DIR`, else the
    /// platform cache dir, and scope it to `language`.
    pub fn from_env(language: &str, ttl: Duration) -> Self {
        let base = std::env::var_os("WEB_READER_CACHE_DIR")
            .map(PathBuf::from)
            .or_else(|| dirs::cache_dir().map(|dir| dir.join("web_reader")))
            .unwrap_or_else(|| PathBuf::from(".web_reader_cache"));
        Self::with_base(base, language, ttl)
    }

    /// Build a cache rooted at `base` for `language`.
    pub fn with_base(base: PathBuf, language: &str, ttl: Duration) -> Self {
        let root = base.join("articles").join(language);
        Self { base, root, ttl }
    }

    /// Configured time-to-live.
    pub fn ttl(&self) -> Duration {
        self.ttl
    }

    /// Root directory holding this language's cached articles.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Shared cache base directory (contains `latest.json`).
    pub fn base(&self) -> &Path {
        &self.base
    }

    /// Path of the cross-language lookup index.
    pub fn latest_path(&self) -> PathBuf {
        self.base.join("latest.json")
    }

    /// Load a cached entry, if present and parseable. The entry may be expired;
    /// callers decide using [`CachedArticle::is_fresh`]. Any I/O or parse error
    /// is treated as a miss.
    pub fn load(&self, title: &str) -> Option<CachedArticle> {
        let meta_raw = fs::read_to_string(self.meta_path(title)).ok()?;
        let meta: ArticleMeta = serde_json::from_str(&meta_raw).ok()?;
        let wikitext = fs::read_to_string(self.wikitext_path(title)).unwrap_or_default();
        Some(CachedArticle { meta, wikitext })
    }

    /// Write both metadata and wikitext for an entry.
    pub fn store(&self, meta: &ArticleMeta, wikitext: &str) -> io::Result<()> {
        let dir = self.entry_dir(&meta.title);
        fs::create_dir_all(&dir)?;
        fs::write(dir.join("meta.json"), to_json(meta)?)?;
        fs::write(dir.join("wikitext.txt"), wikitext)?;
        Ok(())
    }

    /// Rewrite only the metadata, leaving the wikitext payload untouched.
    pub fn update_meta(&self, meta: &ArticleMeta) -> io::Result<()> {
        let dir = self.entry_dir(&meta.title);
        fs::create_dir_all(&dir)?;
        fs::write(dir.join("meta.json"), to_json(meta)?)
    }

    /// Read the lookup index. A missing, unparseable, or wrong-version file
    /// yields an empty index.
    pub fn load_latest(&self) -> LatestIndex {
        match fs::read_to_string(self.latest_path()) {
            Ok(raw) => match serde_json::from_str::<LatestIndex>(&raw) {
                Ok(index) if index.schema == LatestIndex::SCHEMA => index,
                _ => LatestIndex::default(),
            },
            Err(_) => LatestIndex::default(),
        }
    }

    /// Record a successful lookup, moving any existing entry for the same
    /// `(language, title)` to the front. Keeps one row per article.
    pub fn record_latest(&self, entry: LatestEntry) -> io::Result<()> {
        let mut index = self.load_latest();
        index.entries.retain(|existing| {
            !(existing.language == entry.language && existing.title == entry.title)
        });
        index.entries.insert(0, entry);
        fs::create_dir_all(&self.base)?;
        fs::write(self.latest_path(), to_json(&index)?)
    }

    /// The `n` most recent lookups, newest first. Cheap because the index is
    /// already sorted newest-first.
    pub fn recent(&self, n: usize) -> Vec<LatestEntry> {
        let mut entries = self.load_latest().entries;
        entries.truncate(n);
        entries
    }

    fn entry_dir(&self, title: &str) -> PathBuf {
        self.root.join(hash_title(title))
    }

    fn meta_path(&self, title: &str) -> PathBuf {
        self.entry_dir(title).join("meta.json")
    }

    fn wikitext_path(&self, title: &str) -> PathBuf {
        self.entry_dir(title).join("wikitext.txt")
    }
}

/// SHA-256 of the title, hex-encoded. Titles can contain characters that are
/// invalid in filenames, so the payload is addressed by a hash.
fn hash_title(title: &str) -> String {
    let digest = Sha256::digest(title.as_bytes());
    let mut out = String::with_capacity(digest.len() * 2);
    for byte in digest {
        let _ = write!(out, "{byte:02x}");
    }
    out
}

fn to_json<T: Serialize>(value: &T) -> io::Result<Vec<u8>> {
    serde_json::to_vec_pretty(value).map_err(io::Error::other)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use tempfile::TempDir;

    fn at(secs: i64) -> DateTime<Utc> {
        Utc.timestamp_opt(secs, 0).unwrap()
    }

    fn cache(dir: &TempDir, language: &str) -> ArticleCache {
        ArticleCache::with_base(dir.path().to_path_buf(), language, Duration::hours(24))
    }

    fn meta(title: &str, language: &str, revision: u64, expires_at: DateTime<Utc>) -> ArticleMeta {
        ArticleMeta {
            schema: ArticleMeta::SCHEMA,
            title: title.to_string(),
            language: language.to_string(),
            page_id: 1,
            revision_id: revision,
            revision_timestamp: "2020-01-01T00:00:00Z".to_string(),
            license: None,
            cached_at: at(0),
            expires_at,
        }
    }

    fn latest(title: &str, language: &str, revision: u64, looked_up_at: i64) -> LatestEntry {
        LatestEntry {
            language: language.to_string(),
            title: title.to_string(),
            page_id: 1,
            revision_id: revision,
            revision_timestamp: "2020-01-01T00:00:00Z".to_string(),
            looked_up_at: at(looked_up_at),
            from_cache: false,
        }
    }

    #[test]
    fn store_then_load_roundtrips() {
        let dir = TempDir::new().unwrap();
        let cache = cache(&dir, "en");
        let meta = meta("Earth", "en", 42, at(100));

        cache.store(&meta, "the wikitext").unwrap();
        let loaded = cache.load("Earth").expect("entry should load");

        assert_eq!(loaded.meta.revision_id, 42);
        assert_eq!(loaded.wikitext, "the wikitext");
    }

    #[test]
    fn missing_entry_is_a_miss() {
        let dir = TempDir::new().unwrap();
        assert!(cache(&dir, "en").load("Nope").is_none());
    }

    #[test]
    fn freshness_follows_ttl() {
        let dir = TempDir::new().unwrap();
        let cache = cache(&dir, "en");

        let expired = meta("Earth", "en", 42, at(50));
        cache.store(&expired, "text").unwrap();
        assert!(!cache.load("Earth").unwrap().is_fresh(at(100)));

        let fresh = meta("Earth", "en", 42, at(150));
        cache.store(&fresh, "text").unwrap();
        assert!(cache.load("Earth").unwrap().is_fresh(at(100)));
    }

    #[test]
    fn fresh_entry_without_payload_is_not_fresh() {
        let dir = TempDir::new().unwrap();
        let cache = cache(&dir, "en");
        let meta = meta("Earth", "en", 42, at(150));
        cache.store(&meta, "text").unwrap();

        fs::remove_file(cache.wikitext_path("Earth")).unwrap();
        assert!(!cache.load("Earth").unwrap().is_fresh(at(100)));
    }

    #[test]
    fn update_meta_leaves_wikitext_untouched() {
        let dir = TempDir::new().unwrap();
        let cache = cache(&dir, "en");
        cache
            .store(&meta("Earth", "en", 42, at(50)), "original")
            .unwrap();

        cache
            .update_meta(&meta("Earth", "en", 42, at(999)))
            .unwrap();

        let loaded = cache.load("Earth").unwrap();
        assert_eq!(
            loaded.wikitext, "original",
            "wikitext must not be rewritten"
        );
        assert_eq!(loaded.meta.expires_at, at(999), "TTL must be refreshed");
    }

    #[test]
    fn languages_do_not_collide() {
        let dir = TempDir::new().unwrap();
        let en = cache(&dir, "en");
        let de = cache(&dir, "de");
        en.store(&meta("Earth", "en", 1, at(100)), "english")
            .unwrap();
        de.store(&meta("Erde", "de", 2, at(100)), "german").unwrap();

        assert_eq!(en.load("Earth").unwrap().wikitext, "english");
        assert_eq!(de.load("Erde").unwrap().wikitext, "german");
    }

    #[test]
    fn latest_is_newest_first() {
        let dir = TempDir::new().unwrap();
        let cache = cache(&dir, "en");
        cache.record_latest(latest("Earth", "en", 1, 100)).unwrap();
        cache
            .record_latest(latest("Jupiter", "en", 2, 200))
            .unwrap();
        cache.record_latest(latest("Mars", "en", 3, 300)).unwrap();

        let recent = cache.recent(2);
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].title, "Mars");
        assert_eq!(recent[1].title, "Jupiter");
    }

    #[test]
    fn latest_dedupes_and_moves_to_front() {
        let dir = TempDir::new().unwrap();
        let cache = cache(&dir, "en");
        cache.record_latest(latest("Earth", "en", 1, 100)).unwrap();
        cache
            .record_latest(latest("Jupiter", "en", 2, 200))
            .unwrap();
        cache.record_latest(latest("Earth", "en", 9, 300)).unwrap();

        let entries = cache.load_latest().entries;
        assert_eq!(entries.len(), 2, "same article must not duplicate");
        assert_eq!(entries[0].title, "Earth");
        assert_eq!(entries[0].revision_id, 9);
        assert_eq!(entries[1].title, "Jupiter");
    }

    #[test]
    fn latest_is_shared_across_languages() {
        let dir = TempDir::new().unwrap();
        cache(&dir, "en")
            .record_latest(latest("Earth", "en", 1, 100))
            .unwrap();
        cache(&dir, "de")
            .record_latest(latest("Erde", "de", 2, 200))
            .unwrap();

        let entries = cache(&dir, "en").recent(10);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].title, "Erde");
        assert!(entries.iter().any(|e| e.language == "en"));
    }

    #[test]
    fn latest_survives_a_wrong_schema() {
        let dir = TempDir::new().unwrap();
        let cache = cache(&dir, "en");
        cache.record_latest(latest("Earth", "en", 1, 100)).unwrap();

        let raw = fs::read_to_string(cache.latest_path()).unwrap();
        fs::write(
            cache.latest_path(),
            raw.replace("\"schema\": 1", "\"schema\": 99"),
        )
        .unwrap();

        assert!(cache.load_latest().entries.is_empty());
    }
}
