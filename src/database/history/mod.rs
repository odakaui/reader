use super::{state, history_token};
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
        )"#,
        []
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
        r#"INSERT OR IGNORE INTO history (file_id, start_date, current_operation) VALUES (?1, ?2, 0)"#,
        params![file_id, Utc::now()],
    )?;

    let id = conn.last_insert_rowid() as i32;
    current_history::set_current_history(conn, id, file_id)?;

    let history = select_history(conn, id)?;
    state::initial_state(conn, history.id)?;

    Ok(history)
}

fn select_history(conn: &Connection, id: i32) -> Result<History> {
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
        })?)
}

// pub fn update_history_with_current_operation(
//     conn: &Connection,
//     history_id: i32,
//     current_operation: i32,
// ) -> Result<()> {
//     conn.execute(
//         r#"UPDATE history SET current_operation=?1 WHERE history_id=?2"#,
//         params![current_operation, history_id],
//     )?;

//     Ok(())
// }

// pub fn select_history_for_id(conn: &Connection, history_id: i32) -> Result<History> {
//     Ok(conn.query_row(
//         r#"SELECT id, file_id, start_date, end_date, current_operation FROM history WHERE id=?1 ORDER BY id DESC"#,
//         params![history_id],
//         |row| {
//             Ok(History {
//                 id: row.get(0)?,
//                 file_id: row.get(1)?,
//                 start_date: row.get(2)?,
//                 end_date: row.get(3)?,
//                 current_operation: row.get(4)?,
//             })
//         })?)
// }

// pub fn select_current_history(conn: &Connection, file_id: i32) -> Result<History> {
//     Ok(conn.query_row(
//         r#"SELECT id, file_id, start_date, end_date FROM history WHERE file_id=?1 ORDER BY id DESC"#,
//         params![file_id],
//         |row| {
//             Ok(History {
//                 id: row.get(0)?,
//                 file_id: row.get(1)?,
//                 start_date: row.get(2)?,
//                 end_date: row.get(3)?,
//                 current_operation: row.get(4)?,
//             })
//         })?)
// }
