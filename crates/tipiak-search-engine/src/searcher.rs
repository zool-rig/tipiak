use dyn_fmt::AsStrFormatExt;
use rusqlite::{Connection, params_from_iter};
use std::{error::Error, path::Path};

use crate::db::queries::{SEARCH_FILES_QUERY, SELECT_FILE_TYPES_BY_NAMES_QUERY};
use crate::models::from_row::FromRow;
use crate::models::{file::File, file_type::FileType};
use crate::utils::db_utils::{connect, get_db_path};
use crate::utils::token_utils::tokenize_string;

#[derive(Debug)]
pub struct FileTypeFilters {
    pub file_type_names: Vec<String>,
}

impl FileTypeFilters {
    pub fn from_string(file_types: String) -> Self {
        Self {
            file_type_names: file_types.split("|").map(|t| t.to_string()).collect(),
        }
    }

    pub fn file_types(&self, conn: &Connection) -> Result<Vec<FileType>, Box<dyn Error>> {
        if self.file_type_names.is_empty() {
            return Ok(Vec::new());
        }

        let placeholders = std::iter::repeat_n("?", self.file_type_names.len())
            .collect::<Vec<_>>()
            .join(", ");

        let sql = SELECT_FILE_TYPES_BY_NAMES_QUERY.format([&placeholders]);
        let mut stmt = conn.prepare(&sql)?;

        let rows = stmt.query_map(
            params_from_iter(self.file_type_names.iter()),
            FileType::from_row,
        )?;

        Ok(rows.filter_map(Result::ok).collect())
    }
}

pub fn search(
    root_dir: &Path,
    input: &str,
    filters: Option<FileTypeFilters>,
) -> Result<Vec<File>, Box<dyn Error>> {
    let db_path = get_db_path(root_dir);

    if !db_path.exists() {
        return Err(".db file not found".into());
    }

    let inputs: Vec<String> = tokenize_string(input.to_string()).into_iter().collect();

    if inputs.is_empty() {
        return Ok(vec![]);
    }

    let input = inputs.join(" OR ");

    let conn = connect(&db_path)?;

    let sql = match filters {
        Some(f) => SEARCH_FILES_QUERY.format([&format!(
            " AND f.type_id IN ({})",
            f.file_types(&conn)?
                .iter()
                .map(|t| t.id.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )]),
        None => SEARCH_FILES_QUERY.format([""]),
    };

    let mut stmt = conn.prepare(&sql)?;
    Ok(stmt
        .query_map([&input], File::from_row)?
        .filter_map(Result::ok)
        .collect())
}
