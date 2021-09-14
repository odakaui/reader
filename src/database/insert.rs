use anyhow::Result;
use rusqlite::{Connection, params, Statement};

pub fn insert_files(conn: &Connection, files: Vec<File>) -> Result<()> {
    for file in files.iter() {
        conn.execute(r#"INSERT INTO files (name, eof) VALUES (?1, ?2)"#, params![file.name, file.eof])?;
    }

    Ok(())
}
