use super::Database;
use crate::{database::HistoryToken, file::File, History, Token};
use anyhow::Result;
use rusqlite::params;

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

    pub fn select_id_for_token(&self, token: &Token) -> Result<i32> {
        Ok(self.conn.query_row(
            r#"SELECT id FROM tokens WHERE lemma=? AND pos=?"#,
            params![token.lemma, token.pos.to_int()],
            |row| row.get(0),
        )?)
    }

    pub fn select_history_token_for_history_id_and_token_id(
        &self,
        history_id: i32,
        token_id: i32,
    ) -> Result<HistoryToken> {
        Ok(self.conn.query_row(
            r#"SELECT history_id, token_id, total_unknown, total_seen FROM historytokens 
                WHERE history_id=? AND token_id=?"#,
            params![history_id, token_id],
            |row| {
                Ok(HistoryToken::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                ))
            },
        )?)
    }
}
