macro_rules! query {
    ($name:ident, $query:expr) => {
        pub const $name: &str = $query;
    };
}

query!(ENABLE_FOREIGN_KEYS_QUERY, "PRAGMA foreign_keys = ON;");

query!(
    CREATE_FILE_TYPES_TABLE_QUERY,
    "CREATE TABLE file_types (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL UNIQUE
    );"
);

query!(
    CREATE_FILES_TABLE_QUERY,
    "CREATE TABLE files (
        id INTEGER PRIMARY KEY,
        path TEXT NOT NULL UNIQUE,
        type_id INTEGER,
        FOREIGN KEY(type_id) REFERENCES file_types(id) ON DELETE RESTRICT
    );"
);

query!(
    CREATE_TOKENS_TABLE_QUERY,
    "CREATE VIRTUAL TABLE tokens
    USING fts5(
        content,
        file_id UNINDEXED,
        tokenize = 'trigram'
    );"
);

query!(
    INSERT_FILE_TYPES_QUERY,
    "INSERT INTO file_types (name)
    VALUES (?)
    ON CONFLICT(name) DO NOTHING;"
);

query!(
    INSERT_FILE_QUERY,
    "INSERT INTO files (path, type_id)
    VALUES (?, ?)
    ON CONFLICT DO NOTHING;"
);

query!(
    INSERT_TOKENS_QUERY,
    "INSERT INTO tokens (content, file_id)
    VALUES (?, ?);"
);

query!(
    SEARCH_FILES_QUERY,
    "SELECT f.id, f.path, f.type_id, t.name
    FROM tokens
    JOIN files f ON f.id = tokens.file_id
    LEFT JOIN file_types t
    ON f.type_id = t.id
    WHERE tokens MATCH ?{}
    ORDER BY RANK;"
);

query!(
    SELECT_FILE_TYPES_BY_NAMES_QUERY,
    "SELECT * FROM file_types WHERE name IN ({});"
);

query!(SELECT_ALL_TOKENS_QUERY, "SELECT content FROM tokens;");

query!(
    SELECT_PATH_FROM_ID_QUERY,
    "SELECT path FROM files WHERE id = ?;"
);
