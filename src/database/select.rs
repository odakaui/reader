use super::Database;
use crate::{file::File, History};
use anyhow::Result;
use rusqlite::params;
use std::boxed::Box;

impl Database {
    pub fn is_file(&mut self, name: &str) -> Result<bool> {
        Ok(self.select_file_for_name(name).is_ok())
    }

    pub fn select_file_for_name(&self, name: &str) -> Result<File> {
        Ok(self.conn.query_row(
            r#"SELECT id, name FROM files WHERE name=?"#,
            params![name],
            |row| {
                Ok(File {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            },
        )?)
    }

    pub fn select_current_history_for_file(&self, file: &File) -> Result<History> {
        let file_id = file.id;

        Ok(self.conn.query_row(
            r#"SELECT id, file_id, start_date, end_date FROM history WHERE file_id=? ORDER BY id DESC"#,
            params![file_id],
            |row| {
                let end_date = row.get(3).ok();

                Ok(History {
                    id: row.get(0)?,
                    file_id: row.get(1)?,
                    start_date: row.get(2)?,
                    end_date,
                })
            },
        )?)
    }
}
