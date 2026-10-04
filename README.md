# web_reader

Read Wikipedia articles through the [MediaWiki REST API](https://www.mediawiki.org/wiki/API:REST_API)
and turn their wikitext into Markdown.

## Status

MVP. The synchronous REST API client (`search` + fetch latest public revision as
wikitext) works, with an on-disk article cache. The wikitext → Markdown converter is
not written yet; the plan is to use the
[`parse_wiki_text`](https://crates.io/crates/parse_wiki_text) crate.

## Usage

```sh
# Full-text search
cargo run -- search "Rust programming language" --limit 5

# Fetch a page's latest public revision (prints wikitext)
cargo run -- article "Rust (programming language)"

# ...and save it to a file
cargo run -- article "Rust (programming language)" --save article.wikitext

# Show recently looked-up articles
cargo run -- latest --limit 5
```

## Configuration

Wikimedia requires a descriptive `User-Agent` with contact information. The default
points at this repository's URL; override it if you fork or run it elsewhere.

| Setting | Flag | Environment variable | Default |
| --- | --- | --- | --- |
| User-Agent | `--user-agent` | `WEB_READER_USER_AGENT` | `web_reader/<version> (+https://github.com/gojoGS/web_reader)` |
| Language | `--lang` | `WEB_READER_LANG` | `en` |
| Cache directory | — | `WEB_READER_CACHE_DIR` | platform cache dir |
| Cache TTL (seconds) | — | `WEB_READER_CACHE_TTL_SECS` | `86400` (24 h) |

Example:

```sh
WEB_READER_USER_AGENT="my_tool/0.1 (https://example.org; me@example.org)" cargo run -- article Rust
```

## Caching

Fetched articles are cached on disk, keyed per language:

```text
<cache>/articles/<language>/<sha256(title)>/meta.json      # metadata + TTL
<cache>/articles/<language>/<sha256(title)>/wikitext.txt   # payload
<cache>/latest.json                                        # cross-language lookup index
```

- Entries have a **24 h TTL**, stored as an RFC 3339 expiry (`expires_at`).
- Fresh entry → served from cache, no request.
- Expired entry → refetched. Same revision → only `meta.json` (the TTL) is rewritten,
  wikitext is kept. New revision → both files are rewritten.
- Page and revision IDs are local to one wiki (`Earth`/en ≠ `Erde`/de), so the language
  is part of the cache path.
- `latest.json` records successful lookups across all languages, newest-first, one row
  per `(language, title)`. `WikiService::recent_lookups(n)` (the `latest` command) reads
  the first `n` entries directly.

## Tests

Live tests hit the network and are ignored by default:

```sh
cargo test --test live_api -- --ignored
```

## License

Application code: not yet decided. Sample article text under `fixtures/` is
CC BY-SA 4.0, © Wikipedia contributors — see [`fixtures/README.md`](fixtures/README.md).
