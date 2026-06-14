use clap::Args;
use notify::{Event, RecursiveMode, Result, Watcher};
use std::path::PathBuf;
use std::sync::mpsc;
use tipiak_search_engine::crawl;

// Watch a directory and run the crawler when a file is added
#[derive(Args)]
pub struct WatchCommand {
    // The root directory to watch, defaults to current working directory
    #[arg(long, short)]
    path: Option<PathBuf>,
}

impl WatchCommand {
    pub fn run(&self) {
        let path = self.path.clone().unwrap_or(PathBuf::from("."));
        let (tx, rx) = mpsc::channel::<Result<Event>>();
        match notify::recommended_watcher(tx) {
            Ok(mut watcher) => {
                println!("Start watching {:?} ...", path);
                match watcher.watch(&path, RecursiveMode::Recursive) {
                    Ok(()) => {
                        for res in rx {
                            match res {
                                Ok(event) => {
                                    if event.kind.is_create()
                                        && let Err(e) = crawl(&path, false)
                                    {
                                        println!("Failed to crawl directory : {:?}", e);
                                    }
                                }
                                Err(e) => println!("watch error: {:?}", e),
                            }
                        }
                    }
                    Err(e) => println!("Failed to watch {:?} : {:?}", path, e),
                }
            }
            Err(e) => println!("Failed to get recommended watcher : {:?}", e),
        }
    }
}
