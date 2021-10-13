use anyhow::Result;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::{fs, path};

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub id: i32,
    pub name: String,
}

impl File {
    pub fn new(id: i32, name: String) -> Self {
        File { id, name }
    }
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

pub fn load_file(path: &path::Path) -> Result<File> {
    Ok(ron::from_str(&fs::read_to_string(path)?)?)
}

pub fn save_file(path: &path::Path, file: &File) -> Result<()> {
    fs::write(path, &ron::to_string(file)?)?;

    Ok(())
}
