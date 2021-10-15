use anyhow::Result;
use rusqlite::{params, Connection};

use super::token;
use super::Token;

pub struct HistoryToken {
    history_id: i32,
    token_id: i32,
    total_seen: i32,
    total_unknown: i32,
}

pub fn initialize(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS history_tokens (
            history_id INTEGER NOT NULL,
            token_id INTEGER NOT NULL,
            total_seen INTEGER NOT NULL DEFAULT 0,
            total_unknown INTEGER NOT NULL DEFAULT 0,
            PRIMARY KEY (history_id, token_id),
            FOREIGN KEY (history_id) REFERENCES history (id),
            FOREIGN KEY (token_id) REFERENCES token (id)
        )"#,
        [],
    )?;

    token::initialize(conn)?;

    Ok(())
}

pub fn insert_history_tokens(
    conn: &Connection,
    history_id: i32,
    tokens: &Vec<Token>,
    unknown: bool,
) -> Result<()> {
    let unknown = if unknown { 1 } else { 0 };

    for token in tokens.iter() {
        let token_id = match token::select_token_id(conn, &token.lemma) {
            Ok(id) => id,
            Err(_) => {
                token::insert_token(conn, token)?;
                token::select_token_id(conn, &token.lemma)?
            }
        };

        conn.execute(
            r#"INSERT INTO history_tokens (history_id, token_id, total_seen, total_unknown)
                VALUES (?1, ?2, ?3, ?4)
                ON CONFLICT (history_id, token_id) DO UPDATE SET total_seen=total_seen+1, total_unknown=total_unknown+?4"#,
            params![history_id, token_id, 1, unknown]
        )?;
    }

    Ok(())
}

pub fn delete_history_tokens(
    conn: &Connection,
    history_id: i32,
    tokens: &Vec<Token>,
    unknown: bool,
) -> Result<()> {
    let unknown = if unknown { 1 } else { 0 };

    for token in tokens.iter() {
        let token_id = token::select_token_id(conn, &token.lemma)?;

        let history_token = select_history_token(conn, history_id, token_id)?;

        let total_seen = history_token.total_seen - 1;
        let total_unknown = history_token.total_unknown - unknown;

        if total_seen < 1 || total_unknown < 1 {
            conn.execute(
                r#"DELETE FROM history_tokens WHERE history_id=?1 AND token_id=?2"#,
                params![history_id, token_id],
            )?;
        } else {
            conn.execute(
                r#"UPDATE history_tokens SET total_seen=?1, total_unknown=?2 WHERE history_id=?3 AND token_id=?4"#,
                params![total_seen, total_unknown, history_id, token_id]
                )?;
        }
    }

    Ok(())
}

fn select_history_token(conn: &Connection, history_id: i32, token_id: i32) -> Result<HistoryToken> {
    Ok(conn.query_row(
        r#"SELECT history_id, token_id, total_seen, total_unknown FROM history_tokens WHERE history_id=?1 AND token_id=?2"#,
        params![history_id, token_id],
        |row| {
            Ok(HistoryToken {
                history_id: row.get(0)?,
                token_id: row.get(1)?,
                total_seen: row.get(2)?,
                total_unknown: row.get(3)?,
            })
        }
    )?)
}
