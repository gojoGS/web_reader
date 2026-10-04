use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use web_reader::wiki::{self, WikiClient, WikiConfig};

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

    let client = WikiClient::new(config)?;
    match cli.command {
        Commands::Search { query, limit } => run_search(&client, &query, limit),
        Commands::Article { title, save, print } => run_article(&client, &title, save, print),
    }
}

fn run_search(client: &WikiClient, query: &str, limit: u8) -> wiki::Result<()> {
    let results = client.search(query, limit)?;
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

fn run_article(
    client: &WikiClient,
    title: &str,
    save: Option<PathBuf>,
    print: bool,
) -> wiki::Result<()> {
    let page = client.get_page(title)?;
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
