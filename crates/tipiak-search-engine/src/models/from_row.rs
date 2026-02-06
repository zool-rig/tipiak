use rusqlite::Row;

pub trait FromRow {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error>
    where
        Self: Sized;
}
