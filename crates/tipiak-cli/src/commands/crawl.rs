use clap::Args;
use std::path::PathBuf;
use tipiak_search_engine::crawl;

// Crawl the given directory and tokenize files
#[derive(Args)]
pub struct CrawlCommand {
    // The root directory to crawl, defaults to current working directory
    #[arg(long, short)]
    path: Option<PathBuf>,

    // If the sqlite database file exists, delete it
    #[arg(long, short)]
    reset: bool,
}

impl CrawlCommand {
    pub fn run(&self) {
        let path = self.path.clone().unwrap_or(PathBuf::from("."));
        if let Err(e) = crawl(&path, self.reset) {
            println!("Failed to crawl directory : {:?}", e);
        }
    }
}
