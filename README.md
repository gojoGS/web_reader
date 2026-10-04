# web_reader

Read Wikipedia articles through the [MediaWiki REST API](https://www.mediawiki.org/wiki/API:REST_API)
and turn their wikitext into Markdown.

## Status

MVP. Synchronous REST API client (search + fetch latest public revision as wikitext),
an on-disk article cache, and a wikitext → Markdown converter built on
[`parse_wiki_text`](https://crates.io/crates/parse_wiki_text).

## Usage

```sh
# Full-text search: prints "<id>  <title>", a description, and a pasteable URL
cargo run -- search "Rust programming language" --limit 5

# ...as JSON, for scripts (array of result objects, each with `id` and `url`)
cargo run -- search "Rust programming language" --json

# Fetch an article and print it as Markdown. Takes a title, or a URL from search.
cargo run -- article "Rust (programming language)"
cargo run -- article "https://en.wikipedia.org/wiki/Rust_(programming_language)"

# Show recently looked-up articles
cargo run -- latest --limit 5
```

The `search` → `article` round-trip is the intended flow: copy a URL from `search`
(or read `.url` from `--json`) and hand it to `article`. Result URLs are
percent-encoded and browser-ready.

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

## Wikitext → Markdown

`src/markdown.rs` wraps [`parse_wiki_text`](https://crates.io/crates/parse_wiki_text)
and renders its node tree as Markdown (`WikiService::markdown`, `markdown` command).

Handled:

- headings, paragraphs, horizontal rules
- `'''bold'''`, `''italic''`, `'''''both'''''`
- internal links (`[[Page|label]]`, `#fragments`) resolved against the wiki base URL
- external links (`[https://… label]`, bare URLs)
- unordered / ordered / definition lists (with nesting)
- preformatted blocks and `<syntaxhighlight>` / `<code>` / `<math>` tags
- tables (first row as header), character entities

Dropped intentionally (no clean Markdown mapping): templates and their parameters,
`<ref>` citations, categories, images, magic words, comments.

Known limitations: the default `parse_wiki_text` configuration is used (no per-site
configuration yet), so some namespace/extension behaviour is approximate. Dropped
inline templates can leave gaps in prose, and empty dropped citations leave section
headings (e.g. "References") with no body.

The converter is tested against the checked-in fixture
`fixtures/rust_programming_language.wikitext` so no network access is needed; its
rendered output is checked in as `fixtures/rust_programming_language.md`.

## Tests

Live tests hit the network and are ignored by default:

```sh
cargo test --test live_api -- --ignored
```

## License

Application code: not yet decided. Sample article text under `fixtures/` is
CC BY-SA 4.0, © Wikipedia contributors — see [`fixtures/README.md`](fixtures/README.md).
