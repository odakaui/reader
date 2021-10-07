use super::Database;
use crate::file::File;
use anyhow::Result;
use chrono::Utc;
use rusqlite::params;

impl Database {
    // insert a file into the Database
    // does nothing if file with name already exists
    pub fn insert_file(&self, name: &str) -> Result<()> {
        if self.select_file_for_name(name).ok().is_none() {
            self.conn.execute(
                r#"INSERT OR IGNORE INTO files (name) VALUES (?1)"#,
                params![name],
            )?;

            let file = self.select_file_for_name(name)?;

            self.insert_history(file.id)?;
        } else {
            println!("[warning] File already exists.");
        }

        Ok(())
    }

    pub fn insert_history(&self, file_id: i32) -> Result<()> {
        let start_date = Utc::now();

        self.conn.execute(
            r#"INSERT OR IGNORE INTO history (file_id, start_date) VALUES (?1, ?2)"#,
            params![file_id, start_date],
        )?;

        Ok(())
    }
}
