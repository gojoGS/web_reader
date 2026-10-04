# Test fixtures

Sample wikitext captured from the live Wikipedia REST API so the wikitext → Markdown
parser can be developed offline without hammering the API.

| File | Article | Revision | Retrieved |
| --- | --- | --- | --- |
| `rust_programming_language.wikitext` | [Rust (programming language)](https://en.wikipedia.org/wiki/Rust_(programming_language)) | 1375877769 | 2026-10-04 |
| `rust_programming_language.md` | rendered output of the above (reference for the converter) | 1375877769 | 2026-10-04 |

Regenerate the fixtures with:

```sh
cargo run -- article  "Rust (programming language)" --save fixtures/rust_programming_language.wikitext
cargo run -- markdown "Rust (programming language)" --save fixtures/rust_programming_language.md
```

## License

Article text is from Wikipedia and is licensed under
[CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/).
© Wikipedia contributors.
