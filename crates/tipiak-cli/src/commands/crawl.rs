use clap::Args;
use std::path::PathBuf;
use tipiak_search_engine::crawl;

// Crawl the given directory and tokenize files
#[derive(Args)]
pub struct CrawlCommand {
    // The root directory to crawl, defaults to current working directory
    #[arg(long, short)]
    path: Option<PathBuf>,
}

impl CrawlCommand {
    pub fn run(&self) {
        let path = self.path.clone().unwrap_or(PathBuf::from("."));
        if let Err(e) = crawl(&path) {
            println!("Failed to crawl directory : {:?}", e);
        }
    }
}
