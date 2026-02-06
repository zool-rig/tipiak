use crate::models::from_row::FromRow;

#[derive(Debug)]
pub struct File {
    pub id: i64,
    pub path: String,
    pub type_id: i64,
    pub type_name: String,
}

impl FromRow for File {
    fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            path: row.get(1)?,
            type_id: row.get(2)?,
            type_name: row.get(3)?,
        })
    }
}
