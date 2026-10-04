//! Live tests against the Wikipedia REST API.
//!
//! These hit the network, so they are ignored by default. Run them with:
//!
//! ```text
//! cargo test --test live_api -- --ignored
//! ```

use web_reader::wiki::{WikiClient, WikiConfig};

fn client() -> WikiClient {
    WikiClient::new(WikiConfig::default()).expect("build client")
}

#[test]
#[ignore = "hits the live Wikipedia API"]
fn search_returns_results() {
    let results = client()
        .search("Rust programming language", 5)
        .expect("search should succeed");

    assert!(!results.is_empty(), "expected at least one search result");
    assert!(
        results
            .iter()
            .any(|r| r.title.to_lowercase().contains("rust")),
        "expected a result mentioning Rust, got: {results:?}"
    );
}

#[test]
#[ignore = "hits the live Wikipedia API"]
fn fetches_latest_wikitext() {
    let page = client()
        .get_page("Rust (programming language)")
        .expect("page fetch should succeed");

    assert_eq!(page.content_model.as_deref(), Some("wikitext"));
    assert!(page.latest.id > 0, "latest revision id should be set");

    let source = page.source.expect("page should include wikitext source");
    assert!(
        source.contains("Rust"),
        "wikitext should mention Rust ({} bytes)",
        source.len()
    );
}

#[test]
#[ignore = "hits the live Wikipedia API"]
fn encodes_titles_with_special_characters() {
    // Parentheses and spaces must survive percent-encoding.
    let wikitext = client()
        .get_wikitext("Rust (programming language)")
        .expect("wikitext fetch should succeed");
    assert!(wikitext.len() > 1000, "expected a substantial article");
}
