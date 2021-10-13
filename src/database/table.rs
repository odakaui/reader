use anyhow::Result;
use rusqlite::Connection;

pub fn create_files(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
            CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
                )
            "#,
        [],
    )?;

    Ok(())
}

pub fn create_history(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
            CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY,
                start_date TEXT NOT NULL,
                end_date TEXT,
                file_id INTEGER NOT NULL,
                current_operation INTEGER NOT NULL,
                FOREIGN KEY (file_id) REFERENCES files(id)
                );
            "#,
        [],
    )?;

    Ok(())
}

pub fn create_history_tokens(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
            CREATE TABLE IF NOT EXISTS historytokens (
                history_id INTEGER NOT NULL,
                token_id INTEGER NOT NULL,
                total_unknown INTEGER NOT NULL,
                total_seen INTEGER NOT NULL,
                PRIMARY KEY (history_id, token_id)
                );
            "#,
        [],
    )?;

    Ok(())
}

pub fn create_state(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
            CREATE TABLE IF NOT EXISTS state (
                id INTEGER PRIMARY KEY,
                history_id INTEGER NOT NULL,
                current_index INTEGER,
                current_line INTEGER,
                operation_num INTEGER NOT NULL,
                action INTEGER,
                CONSTRAINT uq_state UNIQUE(file_id, operation_num)
                FOREIGN KEY (file_id) REFERENCES files(id)
                );
            "#,
        [],
    )?;

    Ok(())
}

pub fn create_tokens(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
            CREATE TABLE IF NOT EXISTS tokens (
                id INTEGER PRIMARY KEY,
                lemma TEXT NOT NULL,
                pos INTEGER NOT NULL,
                CONSTRAINT uq_token UNIQUE(lemma, pos)
                );
            "#,
        [],
    )?;

    Ok(())
}
