use anyhow::Result;
use rusqlite::{params, Connection};

static KEY: i32 = 0;

pub fn initialize(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS current_file (
            key INTEGER NOT NULL UNIQUE,
            id INTEGER,
            FOREIGN KEY (id) REFERENCES files (id)
        )"#,
        [],
    )?;

    conn.execute(
        r#"INSERT OR IGNORE INTO current_file (key, id) VALUES (?1, ?2)"#,
        params![KEY, None::<i32>],
    )?;

    Ok(())
}

pub fn set_current_file(conn: &Connection, id: i32) -> Result<()> {
    conn.execute(
        r#"UPDATE current_file SET id=?1 WHERE key=?2"#,
        params![id, KEY],
    )?;

    Ok(())
}

pub fn get_current_file(conn: &Connection) -> Result<Option<i32>> {
    Ok(conn.query_row(
        r#"SELECT id FROM current_file WHERE key=?1"#,
        params![KEY],
        |row| Ok(row.get::<usize, Option<i32>>(0)?),
    )?)
}
