use super::Database;
use crate::file::File;
use anyhow::Result;
use rusqlite::params;

impl Database {
    pub fn is_file(&mut self, name: &str) -> Result<bool> {
        if self.select_files_for_name(name)?.is_empty() {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    pub fn select_files_for_name(&mut self, name: &str) -> Result<Vec<File>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, eof FROM files WHERE name=?")?;

        let result = stmt
            .query_map(params![name], |row| {
                Ok(File {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    eof: row.get(2)?,
                })
            })?
            .filter_map(|row| row.ok())
            .collect();

        Ok(result)
    }
}
