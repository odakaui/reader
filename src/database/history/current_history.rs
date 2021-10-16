use anyhow::Result;
use rusqlite::{params, Connection};

pub fn initialize(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS current_history (
            file_id INTEGER NOT NULL UNIQUE,
            id INTEGER NOT NULL,
            FOREIGN KEY (file_id) REFERENCES files (id),
            FOREIGN KEY (id) REFERENCES history (id)
        )"#,
        [],
    )?;

    Ok(())
}

pub fn set_current_history(conn: &Connection, file_id: i32, id: i32) -> Result<()> {
    conn.execute(
        r#"INSERT INTO current_history (file_id, id) VALUES (?1, ?2)
            ON CONFLICT (file_id) DO UPDATE SET id=?2"#,
        params![file_id, id],
    )?;

    Ok(())
}

pub fn get_current_history(conn: &Connection, file_id: i32) -> Result<i32> {
    Ok(conn.query_row(
        r#"SELECT id FROM current_history WHERE file_id=?1"#,
        params![file_id],
        |row| Ok(row.get::<usize, i32>(0)?),
    )?)
}
