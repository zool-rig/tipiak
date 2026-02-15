use rusqlite::Connection;
use std::{
    collections::HashSet,
    error::Error,
    path::{Path, PathBuf},
};

use crate::config::CONFIG;
use crate::constants::DB_NAME;
use crate::db::queries::{ENABLE_FOREIGN_KEYS_QUERY, SELECT_ALL_TOKENS_QUERY};

pub fn connect(db_path: &Path) -> Result<Connection, Box<dyn Error>> {
    let conn = Connection::open(db_path)?;
    conn.execute(ENABLE_FOREIGN_KEYS_QUERY, ())?;
    Ok(conn)
}

pub fn get_db_path(root_dir: &Path) -> PathBuf {
    let mut db_path = match &CONFIG.db_override_path {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(root_dir),
    };
    db_path.push(DB_NAME);
    db_path
}

pub fn get_all_tokens(root_dir: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let db_path = get_db_path(root_dir);
    if !db_path.exists() {
        return Err(format!("Database not found in {:?}", root_dir).into());
    }
    let conn = connect(&db_path)?;
    let mut stmt = conn.prepare(SELECT_ALL_TOKENS_QUERY)?;
    let mut rows = stmt.query([])?;
    let mut tokens: HashSet<String> = HashSet::new();
    while let Some(t) = rows.next()? {
        tokens.extend(
            t.get::<_, String>(0)?
                .split_whitespace()
                .map(|t| t.to_string()),
        );
    }
    Ok(tokens.into_iter().collect())
}
