use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

use web_reader::service::{Article, WikiService};
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
    },
    /// Fetch a page's latest public revision as wikitext.
    Article {
        /// Page title, e.g. "Rust (programming language)".
        title: String,
        /// Save the wikitext to this file.
        #[arg(short, long)]
        save: Option<PathBuf>,
        /// Print the wikitext even when saving to a file.
        #[arg(long)]
        print: bool,
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
        Commands::Search { query, limit } => {
            let results = service.search(&query, limit)?;
            print_search(&query, &results);
            Ok(())
        }
        Commands::Article { title, save, print } => {
            let article = service.article(&title)?;
            emit_article(&article, save, print)
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
        println!("{}", result.title);
        if let Some(description) = result.description.as_deref().filter(|d| !d.is_empty()) {
            println!("    {description}");
        }
    }
}

/// Print an article's revision info, optionally save and/or echo its wikitext.
fn emit_article(article: &Article, save: Option<PathBuf>, print: bool) -> wiki::Result<()> {
    eprintln!(
        "{} — revision {} ({})",
        article.title, article.revision_id, article.revision_timestamp
    );

    if let Some(path) = &save {
        std::fs::write(path, &article.wikitext)?;
        eprintln!(
            "saved {} bytes to {}",
            article.wikitext.len(),
            path.display()
        );
    }
    if print || save.is_none() {
        println!("{}", article.wikitext);
    }
    Ok(())
}
