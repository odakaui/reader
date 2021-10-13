use anyhow::Result;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::{fs, path};
use super::Token;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Line {
    pub sentence: String,
    pub tokens: Vec<Token>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    pub id: i32,
    pub name: String,
    pub lines: Vec<Line>
}

pub fn initialize(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )"#,
        [],
    )?;

    Ok(())
}

pub fn insert_file(conn: &Connection, name: &str) -> Result<()> {
    conn.execute(
        r#"INSERT OR IGNORE INTO files (name) VALUES (?1)"#,
        params![name],
    )?;

    Ok(())
}

pub fn select_file(conn: &Connection, name: &str) -> Result<File> {
    Ok(conn.query_row(
        r#"SELECT id, name FROM files WHERE name=?1"#,
        params![name],
        |row| {
            Ok(File {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        },
    )?)
}

pub fn select_file_for_id(conn: &Connection, id: i32) -> Result<File> {
    Ok(conn.query_row(
        r#"SELECT id, name FROM files WHERE id=?1"#,
        params![id],
        |row| {
            Ok(File {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        },
    )?)
}

pub fn select_files(conn: &Connection) -> Result<Vec<File>> {
    let mut stmt = conn.prepare(r#"SELECT id, name FROM files ORDER BY id ASC"#)?;

    Ok(stmt
        .query_map([], |row| Ok(File::new(row.get(0)?, row.get(1)?)))?
        .flatten()
        .collect())
}

pub fn load_file(path: &path::Path) -> Result<File> {
    Ok(ron::from_str(&fs::read_to_string(path)?)?)
}

pub fn save_file(path: &path::Path, file: &File) -> Result<()> {
    fs::write(path, &ron::to_string(file)?)?;

    Ok(())
}
