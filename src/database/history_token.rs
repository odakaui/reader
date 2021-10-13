use anyhow::Result;
use rusqlite::{params, Connection};

pub struct HistoryToken {
    pub history_id: i32,
    pub token_id: i32,
    pub total_unknown: i32,
    pub total_seen: i32,
}

impl HistoryToken {
    pub fn new(history_id: i32, token_id: i32, total_unknown: i32, total_seen: i32) -> Self {
        HistoryToken {
            history_id,
            token_id,
            total_unknown,
            total_seen,
        }
    }
}

pub fn select_history_tokens(conn: &Connection) -> Result<Vec<HistoryToken>> {
    let stmt = conn
        .prepare(r#"SELECT history_id, token_id, total_unknown, total_seen FROM historytokens"#)?;

    Ok(stmt
        .query_map([], |row| {
            Ok(HistoryToken {
                history_id: row.get(0)?,
                token_id: row.get(1)?,
                total_unknown: row.get(2)?,
                total_seen: row.get(3)?,
            })
        })?
        .flatten()
        .collect())
}

pub fn select_history_tokens_for_history_id(
    conn: &Connection,
    history_id: i32,
) -> Result<Vec<HistoryToken>> {
    let stmt = conn.prepare(
        r#"SELECT history_id, token_id, total_unknown, total_seen FROM historytokens
            WHERE history_id=?1"#,
    )?;

    Ok(stmt
        .query_map(params![history_id], |row| {
            Ok(HistoryToken {
                history_id: row.get(0)?,
                token_id: row.get(1)?,
                total_unknown: row.get(2)?,
                total_seen: row.get(3)?,
            })
        })?
        .flatten()
        .collect())
}
