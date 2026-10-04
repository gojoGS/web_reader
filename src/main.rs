use clap::{Parser, Subcommand};
use std::process::ExitCode;
use web_reader::markdown::MarkdownRenderer;

use web_reader::cache::LatestEntry;
use web_reader::service::WikiService;
use web_reader::wiki::{self, SearchResult, WikiConfig};

#[derive(Debug, Parser)]
#[command(
    name = "web_reader",
    version,
    about = "Read Wikipedia articles through the MediaWiki REST API"
)]
struct Cli {
    /// Override the User-Agent header sent with every request.
    #[arg(long, global = true)]
    user_agent: Option<String>,

    /// Wiki language code, e.g. `en`, `de`, `fr`.
    #[arg(long, global = true)]
    lang: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Search wiki pages.
    Search {
        /// Search terms.
        query: String,
        /// Maximum number of results (1-100).
        #[arg(short, long, default_value_t = 10)]
        limit: u8,
        /// Print the results as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Fetch a page's latest public revision as wikitext.
    Article {
        /// Page title, e.g. "Rust (programming language)".
        title: String,
    },
    /// Show recently looked-up articles.
    Latest {
        /// Maximum number of entries to show.
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> wiki::Result<()> {
    let mut config = WikiConfig::from_env();
    if let Some(user_agent) = cli.user_agent {
        config.user_agent = user_agent;
    }
    if let Some(lang) = cli.lang {
        config.language = lang;
    }

    let service = WikiService::new(config)?;
    match cli.command {
        Commands::Search { query, limit, json } => {
            let results = service.search(&query, limit)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&results)?);
            } else {
                print_search(&query, &results);
            }
            Ok(())
        }
        Commands::Article { title } => {
            let article = service.article(&title)?;
            let renderer = MarkdownRenderer::new("https://en.wikipedia.org/wiki/");
            let markdown = renderer.render(&article.wikitext);
            print!("{}", markdown);
            Ok(())
        }
        Commands::Latest { limit } => {
            print_latest(&service.recent_lookups(limit));
            Ok(())
        }
    }
}

/// Print search results, or a short notice when there are none.
fn print_search(query: &str, results: &[SearchResult]) {
    if results.is_empty() {
        eprintln!("no results for {query:?}");
        return;
    }
    for result in results {
        println!("{}  {}", result.id, result.title);
        if let Some(description) = result.description.as_deref().filter(|d| !d.is_empty()) {
            println!("    {description}");
        }
    }
}

/// Print recently looked-up articles, newest first.
fn print_latest(entries: &[LatestEntry]) {
    if entries.is_empty() {
        eprintln!("no lookups recorded yet");
        return;
    }
    for entry in entries {
        let source = if entry.from_cache { "cache" } else { "api" };
        println!(
            "{} [{}] rev {} ({}) — {} @ {}",
            entry.title,
            entry.language,
            entry.revision_id,
            entry.revision_timestamp,
            source,
            entry.looked_up_at
        );
    }
}
