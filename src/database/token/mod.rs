use anyhow::Result;
use druid::{Data, Lens};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

pub use pos::POS;

pub mod pos;

#[derive(Clone, Debug, PartialEq, Data, Deserialize, Lens, Serialize)]
pub struct Token {
    pub lemma: String,
    pub text: String,
    pub pos: POS,
    pub learned: bool,
}

pub fn initialize(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS tokens (
            id INTEGER PRIMARY KEY,
            lemma TEXT NOT NULL UNIQUE,
            pos INTEGER NOT NULL,
            learned INTEGER NOT NULL DEFAULT 0,
            CONSTRAINT uq_token UNIQUE (lemma, pos)
        )"#,
        [],
    )?;

    Ok(())
}

pub fn insert_token(conn: &Connection, token: &Token) -> Result<()> {
    conn.execute(
        r#"INSERT OR IGNORE INTO tokens (lemma, pos) VALUES (?1, ?2)"#,
        params![token.lemma, token.pos.to_int()],
    )?;

    Ok(())
}

pub fn delete_token(conn: &Connection, lemma: &str) -> Result<()> {
    conn.execute(r#"DELETE FROM tokens WHERE lemma=?1"#, params![lemma])?;

    Ok(())
}

pub fn select_token(conn: &Connection, id: i32) -> Result<Token> {
    Ok(conn.query_row(
        r#"SELECT lemma, pos, learned FROM tokens WHERE id=?1"#,
        params![id],
        |row| {
            Ok(Token {
                lemma: row.get(0)?,
                text: "".to_string(),
                pos: POS::to_pos(row.get(1)?),
                learned: row.get::<usize, i32>(2)? == 1,
            })
        },
    )?)
}

pub fn select_token_id(conn: &Connection, lemma: &str) -> Result<i32> {
    Ok(conn.query_row(
        r#"SELECT id FROM tokens WHERE lemma=?1"#,
        params![lemma],
        |row| row.get::<usize, i32>(0),
    )?)
}

pub fn toggle_learned(conn: &Connection, id: i32) -> Result<bool> {
    let token = select_token(conn, id)?;

    let learned = match token.learned {
        true => false,
        false => true,
    };

    update_token(conn, id, learned)?;

    Ok(learned)
}

fn update_token(conn: &Connection, id: i32, learned: bool) -> Result<()> {
    conn.execute(
        r#"UPDATE tokens SET learned=?1 WHERE id=?2"#,
        params![learned, id],
    )?;

    Ok(())
}
