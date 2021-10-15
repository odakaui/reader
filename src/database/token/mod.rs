use rusqlite::{Connection, params};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub use pos::POS;

pub mod pos;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token {
    pub lemma: String,
    pub text: String,
    pub pos: POS,
}

pub fn initialize(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS tokens (
            id INTEGER PRIMARY KEY,
            lemma TEXT NOT NULL UNIQUE,
            text TEXT NOT NULL,
            pos INTEGER NOT NULL,
        )"#,
        []
    )?;

    Ok(())
}

pub fn select_token_id(conn: &Connection, lemma: &str) -> Result<i32> {
    Ok(conn.query_row(
            r#"SELECT id FROM tokens WHERE lemma=?1"#,
            params![lemma],
            |row| {
                Ok(row.get::<usize, i32>(0)?)
            })?)
}
