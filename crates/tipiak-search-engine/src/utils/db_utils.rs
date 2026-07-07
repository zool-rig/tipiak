use log::info;
use rusqlite::Connection;
use std::{
    collections::HashSet,
    error::Error,
    path::{Path, PathBuf},
};

use crate::config::get_config;
use crate::constants::{CONFIG_PATH_ENV_KEY, DB_NAME};
use crate::db::queries::{
    ENABLE_FOREIGN_KEYS_QUERY, SELECT_ALL_TOKENS_QUERY, SELECT_PATH_FROM_ID_QUERY,
};

pub fn connect(db_path: &Path) -> Result<Connection, Box<dyn Error>> {
    let conn = Connection::open(db_path)?;
    conn.execute(ENABLE_FOREIGN_KEYS_QUERY, ())?;
    Ok(conn)
}

pub fn get_db_path(root_dir: &Path) -> PathBuf {
    let mut db_path = match &get_config().db_override_path {
        Some(path) => {
            info!("Found db_override_path : {:?}", path);
            PathBuf::from(path)
        }
        None => {
            info!("No db_override_path, falling back to : {:?}", root_dir);
            PathBuf::from(root_dir)
        }
    };
    db_path.push(DB_NAME);
    db_path
}

pub fn get_all_tokens(root_dir: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let db_path = get_db_path(root_dir);
    if !db_path.exists() {
        return Err(format!(
            "Database not found. root_dir={:?}, config={:?}, env={:?}",
            root_dir,
            get_config(),
            std::env::var(CONFIG_PATH_ENV_KEY)
        )
        .into());
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

pub fn get_path_from_id(root_dir: &Path, id: i64) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let db_path = get_db_path(root_dir);
    if !db_path.exists() {
        return Err(format!(
            "Database not found. root_dir={:?}, config={:?}, env={:?}",
            root_dir,
            get_config(),
            std::env::var(CONFIG_PATH_ENV_KEY)
        )
        .into());
    }
    let conn = connect(&db_path)?;
    let mut stmt = conn.prepare(SELECT_PATH_FROM_ID_QUERY)?;
    let mut rows = stmt.query([id])?;
    let mut path: Option<PathBuf> = None;
    while let Some(p) = rows.next()? {
        // path = Some(fs::canonicalize(PathBuf::from(p.get::<_, String>(0)?))?)
        path = Some(PathBuf::from(p.get::<_, String>(0)?))
    }
    Ok(path)
}
