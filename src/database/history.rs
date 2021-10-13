use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};

pub struct History {
    pub id: i32,
    pub file_id: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub current_operation: i32,
}

impl History {
    pub fn new(
        id: i32,
        file_id: i32,
        start_date: DateTime<Utc>,
        end_date: Option<DateTime<Utc>>,
        current_operation: i32,
    ) -> Self {
        History {
            id,
            file_id,
            start_date,
            end_date,
            current_operation,
        }
    }
}

pub fn insert_history(conn: &Connection, file_id: i32) -> Result<()> {
    conn.execute(
        r#"INSERT OR IGNORE INTO history(file_id, start_date, current_operation) VALUES (?1, ?2, 0)"#,
        params![file_id, Utc::now()],
    )?;

    Ok(())
}

pub fn update_history_with_current_operation(
    conn: &Connection,
    history_id: i32,
    current_operation: i32,
) -> Result<()> {
    conn.execute(
        r#"UPDATE history SET current_operation=?1 WHERE history_id=?2"#,
        params![current_operation, history_id],
    )?;

    Ok(())
}
pub fn select_history(conn: &Connection, file_id: i32) -> Result<Vec<History>> {
    let mut stmt = conn.prepare(
        r#"SELECT id, file_id, start_date, end_date, current_operation FROM history WHERE file_id=?1"#,
    )?;

    Ok(stmt
        .query_map(params![file_id], |row| {
            Ok(History {
                id: row.get(0)?,
                file_id: row.get(1)?,
                start_date: row.get(2)?,
                end_date: row.get(3)?,
                current_operation: row.get(4)?,
            })
        })?
        .flatten()
        .collect())
}

pub fn select_history_for_id(conn: &Connection, history_id: i32) -> Result<History> {
    Ok(conn.query_row(
        r#"SELECT id, file_id, start_date, end_date, current_operation FROM history WHERE id=?1 ORDER BY id DESC"#,
        params![history_id],
        |row| {
            Ok(History {
                id: row.get(0)?,
                file_id: row.get(1)?,
                start_date: row.get(2)?,
                end_date: row.get(3)?,
                current_operation: row.get(4)?,
            })
        })?)
}

pub fn select_current_history(conn: &Connection, file_id: i32) -> Result<History> {
    Ok(conn.query_row(
        r#"SELECT id, file_id, start_date, end_date FROM history WHERE file_id=?1 ORDER BY id DESC"#,
        params![file_id],
        |row| {
            Ok(History {
                id: row.get(0)?,
                file_id: row.get(1)?,
                start_date: row.get(2)?,
                end_date: row.get(3)?,
                current_operation: row.get(4)?,
            })
        })?)
}
