use crate::models::from_row::FromRow;

pub struct FileType {
    pub id: i64,
    pub name: String,
}

impl FromRow for FileType {
    fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }
}
