use rusqlite::{params, Connection};
use anyhow::Result;

pub fn initialize(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS current_state (
            history_id INTEGER NOT NULL UNIQUE,
            id INTEGER NOT NULL
        )"#,
        []
    )?;

    Ok(())
}

pub fn set_current_state(conn: &Connection, history_id: i32, id: i32) -> Result<()> {
    conn.execute(
        r#"INSERT INTO current_state (history_id, id) VALUES (?1, ?2)
            ON CONFLICT (history_id) DO UPDATE SET id=?2"#,
        params![history_id, id]
    )?;

    Ok(())
}

pub fn get_current_state(conn: &Connection, history_id: i32) -> Result<i32> {
    Ok(conn.query_row(
        r#"SELECT id FROM current_state WHERE history_id=?1"#,
        params![history_id],
        |row| {
            Ok(row.get::<usize, i32>(0)?)
        })?
    )
}

pub fn delete_current_state(conn: &Connection, history_id: i32) -> Result<()> {
    conn.execute(
        r#"DELETE FROM current_state WHERE history_id=?1"#,
        params![history_id])?;

    Ok(())
}
