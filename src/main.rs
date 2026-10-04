use clap::{Parser, Subcommand};
use std::process::ExitCode;

use web_reader::service::WikiService;
use web_reader::wiki::{self, WikiConfig};

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
        save: Option<std::path::PathBuf>,
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
        Commands::Search { query, limit } => service.search(&query, limit),
        Commands::Article { title, save, print } => service.article(&title, save, print),
    }
}
