use log::{info, warn};
use simple_logger::SimpleLogger;
use std::{
    collections::HashMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use crate::config::CONFIG;
use crate::constants::DB_NAME;
use crate::db::queries::{
    CREATE_FILE_TYPES_TABLE_QUERY, CREATE_FILES_TABLE_QUERY, CREATE_TOKENS_TABLE_QUERY,
    INSERT_FILE_QUERY, INSERT_FILE_TYPES_QUERY, INSERT_TOKENS_QUERY,
};
use crate::tokenizers::registry::TokenizerRegistry;
use crate::utils::db_utils::{connect, get_db_path};

fn get_file_type_from_ext(ext: String) -> Option<String> {
    for (key, value) in CONFIG.file_types.iter() {
        if value.contains(&ext.to_lowercase()) {
            return Some(key.clone());
        }
    }
    None
}

#[derive(Debug)]
struct CrawlTask {
    id: i64,
    path: PathBuf,
}

pub fn crawl(root_dir: &Path) -> Result<(), Box<dyn Error>> {
    let _ = SimpleLogger::new().init();

    let db_path = get_db_path(root_dir);
    let db_exists = db_path.exists();

    let mut conn = connect(&db_path)?;

    if !db_exists {
        let mut sql = String::from("BEGIN;");
        sql.push_str(&format!("\n{}", CREATE_FILE_TYPES_TABLE_QUERY));
        sql.push_str(&format!("\n{}", CREATE_FILES_TABLE_QUERY));
        sql.push_str(&format!("\n{}", CREATE_TOKENS_TABLE_QUERY));
        sql.push_str("\nCOMMIT;");
        conn.execute_batch(&sql)?;
    }

    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare(INSERT_FILE_TYPES_QUERY)?;
        for file_type in CONFIG.file_types.keys() {
            stmt.execute([file_type])?;
        }
    }
    tx.commit()?;

    let mut stmt = conn.prepare("SELECT name, id FROM file_types;")?;
    let file_type_map: HashMap<String, i64> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
        .flatten()
        .collect();

    let mut files_to_tokenize: Vec<CrawlTask> = vec![];

    for entry in WalkDir::new(root_dir)
        .follow_links(false)
        .into_iter()
        .flatten()
    {
        let path = entry.path().to_owned();
        if !path.is_file() || entry.file_name() == DB_NAME || path.to_str().is_none() {
            continue;
        }

        match path.extension() {
            Some(ext) => match ext.to_str() {
                Some(ext) => match get_file_type_from_ext(ext.to_string()) {
                    Some(file_type_name) => {
                        let file_type_id = file_type_map.get(&file_type_name).unwrap();
                        let inserted = conn.execute(
                            INSERT_FILE_QUERY,
                            (fs::canonicalize(&path)?.to_str(), file_type_id),
                        )?;
                        if inserted == 0 {
                            continue;
                        }

                        files_to_tokenize.push(CrawlTask {
                            id: conn.last_insert_rowid(),
                            path,
                        })
                    }
                    None => warn!("Unsupported extension : {ext}"),
                },
                None => warn!("Can't convert {:?} to &str", ext),
            },
            None => warn!("No extension found for {:?}", entry.path()),
        }
    }

    if files_to_tokenize.is_empty() {
        info!("No new files found");
        return Ok(());
    }

    info!("Found {} new files to tokenize", files_to_tokenize.len());

    let registry = TokenizerRegistry::new();

    for task in files_to_tokenize {
        let tokens = registry.tokenize(&task.path)?;
        if tokens.is_empty() {
            warn!("No tokens found for {:?}", task.path);
            continue;
        }
        conn.execute(INSERT_TOKENS_QUERY, (tokens.join(" "), task.id))?;
        info!("{:?} tokenized !", task.path);
    }

    Ok(())
}
