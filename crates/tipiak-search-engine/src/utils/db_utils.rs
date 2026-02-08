use rusqlite::Connection;
use std::{
    error::Error,
    path::{Path, PathBuf},
};

use crate::config::CONFIG;
use crate::constants::DB_NAME;
use crate::db::queries::ENABLE_FOREIGN_KEYS_QUERY;

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
