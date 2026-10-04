//! Wikitext → Markdown conversion, built on [`parse_wiki_text`].
//!
//! This is a pragmatic rendering, not a perfect one: it targets readable
//! Markdown from typical Wikipedia articles and drops constructs that don't map
//! cleanly (templates, categories, `<ref>` citations, images, magic words).
//!
//! [`parse_wiki_text`]: https://crates.io/crates/parse_wiki_text

use parse_wiki_text::{
    Configuration, DefinitionListItem, DefinitionListItemType, ListItem, Node, TableCaption,
    TableRow,
};

use crate::wiki::encode_title;

/// Converts wikitext into Markdown, resolving wiki links against `wiki_base`
/// (for example `https://en.wikipedia.org/wiki/`).
#[derive(Debug, Clone)]
pub struct MarkdownRenderer {
    wiki_base: String,
}

impl MarkdownRenderer {
    /// Create a renderer whose internal links point at `wiki_base`.
    pub fn new(wiki_base: impl Into<String>) -> Self {
        Self {
            wiki_base: wiki_base.into(),
        }
    }

    /// Parse `wikitext` and render it as Markdown.
    pub fn render(&self, wikitext: &str) -> String {
        let output = Configuration::default().parse(wikitext);
        let mut buffer = String::with_capacity(wikitext.len());
        render_nodes(&output.nodes, &mut buffer, &self.wiki_base);
        cleanup(&buffer)
    }
}

/// Convenience wrapper around [`MarkdownRenderer`].
pub fn to_markdown(wikitext: &str, wiki_base: &str) -> String {
    MarkdownRenderer::new(wiki_base).render(wikitext)
}

fn render_nodes(nodes: &[Node], out: &mut String, base: &str) {
    for node in nodes {
        render_node(node, out, base);
    }
}

fn render_inline(nodes: &[Node], base: &str) -> String {
    let mut buffer = String::new();
    render_nodes(nodes, &mut buffer, base);
    buffer.trim().to_string()
}

fn render_node(node: &Node, out: &mut String, base: &str) {
    match node {
        Node::Text { value, .. } => out.push_str(&value.replace('\n', " ")),
        Node::CharacterEntity { character, .. } => out.push(*character),
        // Formatting is expressed as toggles with no children.
        Node::Bold { .. } => out.push_str("**"),
        Node::Italic { .. } => out.push('*'),
        Node::BoldItalic { .. } => out.push_str("***"),
        Node::Heading { level, nodes, .. } => {
            out.push_str("\n\n");
            for _ in 0..*level {
                out.push('#');
            }
            out.push(' ');
            render_nodes(nodes, out, base);
            out.push_str("\n\n");
        }
        Node::ParagraphBreak { .. } => out.push_str("\n\n"),
        Node::HorizontalDivider { .. } => out.push_str("\n\n---\n\n"),
        Node::Link { target, text, .. } => {
            let rendered = render_inline(text, base);
            let label = if rendered.is_empty() {
                *target
            } else {
                rendered.as_str()
            };
            out.push('[');
            out.push_str(label);
            out.push_str("](");
            out.push_str(&link_url(base, target));
            out.push(')');
        }
        Node::ExternalLink { nodes, .. } => {
            let raw = collect_text(nodes);
            match raw.split_once(char::is_whitespace) {
                Some((url, label)) if !label.trim().is_empty() => {
                    out.push('[');
                    out.push_str(label.trim());
                    out.push_str("](");
                    out.push_str(url.trim());
                    out.push(')');
                }
                _ => {
                    out.push('<');
                    out.push_str(raw.trim());
                    out.push('>');
                }
            }
        }
        Node::UnorderedList { items, .. } => render_list(items, "-", 0, out, base),
        Node::OrderedList { items, .. } => render_list(items, "1.", 0, out, base),
        Node::DefinitionList { items, .. } => render_definition_list(items, 0, out, base),
        Node::Preformatted { nodes, .. } => {
            out.push_str("\n\n```\n");
            out.push_str(&collect_text(nodes));
            if !out.ends_with('\n') {
                out.push('\n');
            }
            out.push_str("```\n\n");
        }
        Node::Table { rows, captions, .. } => render_table(rows, captions, out, base),
        Node::Tag { name, nodes, .. } => render_extension_tag(name, nodes, out, base),
        Node::StartTag { name, .. } => {
            if let Some(marker) = start_tag_marker(name) {
                out.push_str(marker);
            }
        }
        Node::EndTag { name, .. } => {
            if let Some(marker) = end_tag_marker(name) {
                out.push_str(marker);
            }
        }
        Node::Redirect { target, .. } => {
            out.push_str("\n\n*Redirect to ");
            out.push('[');
            out.push_str(target);
            out.push_str("](");
            out.push_str(&link_url(base, target));
            out.push_str(")*\n\n");
        }
        // Dropped: not expressible in Markdown, or unwanted noise.
        Node::Image { .. }
        | Node::Category { .. }
        | Node::Template { .. }
        | Node::Parameter { .. }
        | Node::MagicWord { .. }
        | Node::Comment { .. } => {}
    }
}

fn render_list(items: &[ListItem], marker: &str, indent: usize, out: &mut String, base: &str) {
    for item in items {
        let mut content = String::new();
        for node in &item.nodes {
            if !is_list(node) {
                render_node(node, &mut content, base);
            }
        }
        let has_nested = item.nodes.iter().any(is_list);
        if content.trim().is_empty() && !has_nested {
            continue;
        }
        out.push('\n');
        out.push_str(&" ".repeat(indent));
        out.push_str(marker);
        out.push(' ');
        out.push_str(content.trim());
        for node in &item.nodes {
            match node {
                Node::UnorderedList { items, .. } => render_list(items, "-", indent + 2, out, base),
                Node::OrderedList { items, .. } => render_list(items, "1.", indent + 2, out, base),
                Node::DefinitionList { items, .. } => {
                    render_definition_list(items, indent + 2, out, base)
                }
                _ => {}
            }
        }
    }
    out.push('\n');
}

fn render_definition_list(
    items: &[DefinitionListItem],
    indent: usize,
    out: &mut String,
    base: &str,
) {
    for item in items {
        let mut content = String::new();
        render_nodes(&item.nodes, &mut content, base);
        if content.trim().is_empty() {
            continue;
        }
        out.push('\n');
        out.push_str(&" ".repeat(indent));
        match item.type_ {
            DefinitionListItemType::Term => {
                out.push_str("**");
                out.push_str(content.trim());
                out.push_str("**");
            }
            DefinitionListItemType::Details => {
                out.push_str(": ");
                out.push_str(content.trim());
            }
        }
    }
    out.push('\n');
}

fn render_table(rows: &[TableRow], captions: &[TableCaption], out: &mut String, base: &str) {
    out.push_str("\n\n");
    for caption in captions {
        out.push('*');
        out.push_str(&render_inline(&caption.content, base));
        out.push_str("*\n\n");
    }

    let mut iter = rows.iter();
    let Some(header) = iter.next() else {
        out.push('\n');
        return;
    };

    let header_cells = row_cells(header, base);
    push_table_row(&header_cells, out);
    out.push('|');
    for _ in &header_cells {
        out.push_str(" --- |");
    }
    out.push('\n');

    for row in iter {
        let cells = row_cells(row, base);
        // Dropped templates leave rows with no real cells; don't emit bare `|`.
        if cells.iter().all(|cell| cell.trim().is_empty()) {
            continue;
        }
        push_table_row(&cells, out);
    }
    out.push('\n');
}

fn row_cells(row: &TableRow, base: &str) -> Vec<String> {
    row.cells
        .iter()
        .map(|cell| {
            render_inline(&cell.content, base)
                .replace('|', "\\|")
                .replace('\n', " ")
        })
        .collect()
}

fn push_table_row(cells: &[String], out: &mut String) {
    out.push('|');
    for cell in cells {
        out.push(' ');
        out.push_str(cell);
        out.push_str(" |");
    }
    out.push('\n');
}

fn render_extension_tag(name: &str, nodes: &[Node], out: &mut String, base: &str) {
    match name {
        // Citation/apparatus tags whose content is not article prose.
        "ref" | "references" | "gallery" | "timeline" | "score" | "graph" | "mapframe"
        | "maplink" | "chem" | "ce" => {}
        "nowiki" => out.push_str(&collect_text(nodes)),
        "math" => {
            out.push_str("\n\n$$\n");
            out.push_str(collect_text(nodes).trim());
            out.push_str("\n$$\n\n");
        }
        "code" | "syntaxhighlight" | "source" | "pre" => {
            out.push_str("\n\n```\n");
            out.push_str(collect_text(nodes).trim_end_matches('\n'));
            out.push_str("\n```\n\n");
        }
        _ => render_nodes(nodes, out, base),
    }
}

fn start_tag_marker(name: &str) -> Option<&'static str> {
    match name {
        "b" | "strong" => Some("**"),
        "i" | "em" => Some("*"),
        "code" | "tt" | "kbd" | "samp" => Some("`"),
        "br" => Some("  \n"),
        "hr" => Some("\n\n---\n\n"),
        _ => None,
    }
}

fn end_tag_marker(name: &str) -> Option<&'static str> {
    match name {
        "b" | "strong" => Some("**"),
        "i" | "em" => Some("*"),
        "code" | "tt" | "kbd" | "samp" => Some("`"),
        _ => None,
    }
}

fn is_list(node: &Node) -> bool {
    matches!(
        node,
        Node::UnorderedList { .. } | Node::OrderedList { .. } | Node::DefinitionList { .. }
    )
}

/// Concatenate the literal text of a node list, ignoring formatting.
fn collect_text(nodes: &[Node]) -> String {
    let mut text = String::new();
    for node in nodes {
        match node {
            Node::Text { value, .. } => text.push_str(value),
            Node::CharacterEntity { character, .. } => text.push(*character),
            Node::Tag { nodes, .. } => text.push_str(&collect_text(nodes)),
            _ => {}
        }
    }
    text
}

/// Build a wiki URL for an internal link target.
fn link_url(base: &str, target: &str) -> String {
    if target.contains("://") {
        return target.to_string();
    }
    // A leading colon means "link to the page, not the namespace".
    let target = target.trim_start_matches(':');
    let (page, fragment) = match target.split_once('#') {
        Some((page, fragment)) => (page, Some(fragment)),
        None => (target, None),
    };
    let mut url = format!("{base}{}", encode_title(page));
    if let Some(fragment) = fragment {
        url.push('#');
        url.push_str(&encode_title(fragment));
    }
    url
}

/// Collapse blank-line runs, squeeze runs of spaces, trim trailing spaces, and
/// end with a single newline. Fenced code blocks are preserved verbatim.
fn cleanup(input: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut pending_blank = false;
    let mut in_fence = false;
    for raw in input.lines() {
        let line = raw.trim_end();
        let is_fence = line.trim_start().starts_with("```");
        if in_fence || is_fence {
            lines.push(line.to_string());
            if is_fence {
                in_fence = !in_fence;
            }
            continue;
        }
        let collapsed = collapse_spaces(line);
        if collapsed.is_empty() {
            pending_blank = true;
            continue;
        }
        if pending_blank && !lines.is_empty() {
            lines.push(String::new());
        }
        pending_blank = false;
        lines.push(trim_stray_indent(collapsed));
    }

    let mut out = lines.join("\n");
    while out.ends_with('\n') {
        out.pop();
    }
    if !out.is_empty() {
        out.push('\n');
    }
    out
}

/// Reduce runs of spaces to one, keeping leading indentation intact.
fn collapse_spaces(line: &str) -> String {
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];
    let mut out = String::with_capacity(line.len());
    let mut previous_space = false;
    for ch in trimmed.chars() {
        if ch == ' ' {
            if !previous_space {
                out.push(' ');
            }
            previous_space = true;
        } else {
            out.push(ch);
            previous_space = false;
        }
    }
    format!("{indent}{out}")
}

/// Drop leading whitespace unless it is meaningful indentation that we emit
/// ourselves (nested list items, definition lists). Stray leading spaces come
/// from blank lines around dropped templates and would otherwise turn a
/// paragraph into a Markdown code block.
fn trim_stray_indent(line: String) -> String {
    let trimmed = line.trim_start();
    if preserves_indent(trimmed) {
        line
    } else {
        trimmed.to_string()
    }
}

fn preserves_indent(trimmed: &str) -> bool {
    trimmed.starts_with("- ")
        || trimmed.starts_with("* ")
        || trimmed.starts_with("+ ")
        || trimmed.starts_with(": ")
        || trimmed.starts_with("> ")
        || trimmed.starts_with('|')
        || has_ordered_marker(trimmed)
}

/// Matches our ordered-list marker, `1. `.
fn has_ordered_marker(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut digits = 0;
    while digits < bytes.len() && bytes[digits].is_ascii_digit() {
        digits += 1;
    }
    digits > 0 && bytes.get(digits) == Some(&b'.') && bytes.get(digits + 1) == Some(&b' ')
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = "https://en.wikipedia.org/wiki/";

    fn md(input: &str) -> String {
        to_markdown(input, BASE)
    }

    #[test]
    fn headings() {
        assert_eq!(md("== Title ==\nBody."), "## Title\n\nBody.\n");
    }

    #[test]
    fn bold_italic() {
        assert_eq!(md("'''bold''' and ''italic''"), "**bold** and *italic*\n");
        assert_eq!(md("'''''both'''''"), "***both***\n");
    }

    #[test]
    fn internal_links() {
        assert_eq!(
            md("[[Rust (programming language)|Rust]]"),
            "[Rust](https://en.wikipedia.org/wiki/Rust_(programming_language))\n"
        );
        assert_eq!(
            md("[[Earth]]"),
            "[Earth](https://en.wikipedia.org/wiki/Earth)\n"
        );
        assert_eq!(
            md("[[Earth#Orbit|orbit]]"),
            "[orbit](https://en.wikipedia.org/wiki/Earth#Orbit)\n"
        );
    }

    #[test]
    fn external_links() {
        assert_eq!(
            md("See [https://example.com Example]."),
            "See [Example](https://example.com).\n"
        );
        assert_eq!(
            md("See [https://example.com]."),
            "See <https://example.com>.\n"
        );
    }

    #[test]
    fn lists() {
        let input = "* one\n* two\n** nested\n";
        assert_eq!(md(input), "- one\n- two\n  - nested\n");
    }

    #[test]
    fn templates_and_refs_are_dropped() {
        assert_eq!(md("A {{Infobox|x=1}} B<ref>citation</ref> C"), "A B C\n");
    }

    #[test]
    fn inline_code_and_nowiki() {
        assert_eq!(md("<code>let x = 1;</code>"), "`let x = 1;`\n");
        assert_eq!(md("<nowiki>{{raw}}</nowiki>"), "{{raw}}\n");
    }

    #[test]
    fn horizontal_divider_and_entities() {
        assert_eq!(md("a\n----\nb"), "a\n\n---\n\nb\n");
        assert_eq!(md("&lt; &amp; &ouml;"), "< & ö\n");
    }

    #[test]
    fn categories_are_dropped() {
        assert_eq!(md("Text\n[[Category:Programming]]"), "Text\n");
    }

    #[test]
    fn preformatted_becomes_code_block() {
        let output = md(" preformatted\n line");
        assert!(output.starts_with("```\n"), "got: {output:?}");
        assert!(output.contains("preformatted\n"));
    }

    #[test]
    fn leading_whitespace_is_trimmed() {
        // Dropped templates leave blank lines that used to push the paragraph
        // into a 4-space Markdown code block.
        let output = md(
            "{{Short description|x}}\n{{good article}}\n{{Use mdy dates|date=x}}\n\n'''Title''' is a thing.",
        );
        assert!(
            !output.lines().any(|line| line.starts_with(' ')),
            "unexpected indentation: {output:?}"
        );
        assert!(output.starts_with("**Title**"), "got: {output:?}");
    }

    #[test]
    fn table_rows_without_cells_are_skipped() {
        let input =
            "{|\n! A !! B\n|-\n| x || y\n|-\n{{album chart|Australia|47}}\n|-\n| z || w\n|}";
        let output = md(input);
        assert!(output.contains("| x | y |"), "got: {output:?}");
        assert!(output.contains("| z | w |"), "got: {output:?}");
        assert!(
            !output.lines().any(|line| line.trim() == "|"),
            "empty rows leaked: {output:?}"
        );
    }

    #[test]
    fn renders_the_real_article_fixture() {
        let wikitext = include_str!("../fixtures/rust_programming_language.wikitext");
        let output = to_markdown(wikitext, BASE);

        assert!(output.len() > 5_000, "expected substantial output");
        assert!(output.contains("## "), "expected headings");
        assert!(output.contains("]("), "expected links");
        assert!(
            !output.contains("{{Infobox"),
            "templates should be stripped"
        );
        assert!(!output.contains("<ref"), "citation tags should be stripped");
        assert!(
            !output.contains("[[Category:"),
            "categories should be stripped"
        );
    }
}
