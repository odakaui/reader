use super::Database;
use crate::file::File;
use anyhow::Result;
use rusqlite::{params, Connection};

impl Database {
    pub fn insert_file(&self, file: &File) -> Result<()> {
        self.insert_files(vec![file])?;

        Ok(())
    }

    pub fn insert_files(&self, files: Vec<&File>) -> Result<()> {
        for file in files.iter() {
            self.conn.execute(
                r#"INSERT OR IGNORE INTO files (name, eof) VALUES (?1, ?2)"#,
                params![file.name, file.eof],
            )?;
        }

        Ok(())
    }
}
