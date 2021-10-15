use super::{history_token, state};
use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};

mod current_history;

pub struct History {
    pub id: i32,
    pub file_id: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}

pub fn initialize(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS history (
            id INTEGER PRIMARY KEY,
            file_id INTEGER NOT NULL,
            start_date TEXT NOT NULL,
            end_date TEXT,
            FOREIGN KEY (file_id) REFERENCES files (id)
        )"#,
        [],
    )?;

    current_history::initialize(conn)?;
    state::initialize(conn)?;
    history_token::initialize(conn)?;

    Ok(())
}

pub fn current_history(conn: &Connection, file_id: i32) -> Result<i32> {
    Ok(current_history::get_current_history(conn, file_id)?)
}

pub fn reset_history(conn: &Connection, file_id: i32, id: i32) -> Result<()> {
    state::reset_state(conn, id)?;

    insert_history(conn, file_id)?;

    Ok(())
}

pub fn insert_history(conn: &Connection, file_id: i32) -> Result<History> {
    conn.execute(
        r#"INSERT OR IGNORE INTO history (file_id, start_date) VALUES (?1, ?2)"#,
        params![file_id, Utc::now()],
    )?;

    let id = conn.last_insert_rowid() as i32;
    current_history::set_current_history(conn, id, file_id)?;

    let history = select_history(conn, id)?;
    state::initial_state(conn, history.id)?;

    Ok(history)
}

pub fn select_history(conn: &Connection, id: i32) -> Result<History> {
    Ok(conn.query_row(
        r#"SELECT id, file_id, start_date, end_date FROM history WHERE id=?1"#,
        params![id],
        |row| {
            Ok(History {
                id: row.get(0)?,
                file_id: row.get(1)?,
                start_date: row.get(2)?,
                end_date: row.get(3)?,
            })
        },
    )?)
}
