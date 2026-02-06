mod commands;

use clap::{Parser, Subcommand};

use crate::commands::{crawl::CrawlCommand, search::SearchCommand, watch::WatchCommand};

#[derive(Parser)]
#[command(
    name = "tipiak-cli",
    version = "0.1.0",
    about = "Tipiak search engine's command line client"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Crawl(CrawlCommand),
    Search(SearchCommand),
    Watch(WatchCommand),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Crawl(cmd) => cmd.run(),
        Commands::Search(cmd) => cmd.run(),
        Commands::Watch(cmd) => cmd.run(),
    }
}
