use clap::Args;
use std::path::PathBuf;
use tipiak_search_engine::{FileTypeFilters, search};

// Search files from key words
#[derive(Args)]
pub struct SearchCommand {
    // Pattern to search
    pattern: String,

    // File types to include separated by '|', ex : images|videos|sounds
    #[arg(long, short)]
    filters: Option<String>,

    // The root directory to search on, defaults to current working directory
    #[arg(long, short)]
    path: Option<PathBuf>,
}

impl SearchCommand {
    pub fn run(&self) {
        let path = self.path.clone().unwrap_or(PathBuf::from("."));
        let filters = self
            .filters
            .as_ref()
            .map(|f| FileTypeFilters::from_string(f.to_string()));
        match search(&path, &self.pattern, filters) {
            Ok(files) => {
                for file in files {
                    println!("{}", file.path);
                }
            }
            Err(e) => println!(
                "Failed to search {:?} in {:?} : {:?}",
                path, self.pattern, e
            ),
        }
    }
}
