use super::{Database, HistoryToken};
use crate::{State, History, Token};
use anyhow::{Result, anyhow};
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

    pub fn insert_tokens_for_history(
        &self,
        history: &History,
        tokens: Vec<Token>,
        is_unknown: bool,
    ) -> Result<()> {
        self.insert_tokens(tokens.clone())?;

        let history_id = history.id;

        for token in tokens.iter() {
            let token_id = self.select_id_for_token(token)?;

            match self.select_history_token_for_history_id_and_token_id(history_id, token_id) {
                Ok(mut history_token) => {
                    history_token.total_seen += 1;

                    if is_unknown {
                        history_token.total_unknown += 1;
                    }

                    self.update_history_token(&history_token)?;
                }
                Err(_) => {
                    let total_unknown = if is_unknown { 1 } else { 0 };
                    let history_token = HistoryToken::new(history_id, token_id, total_unknown, 1);

                    self.insert_history_token(&history_token)?;
                }
            }
        }

        Ok(())
    }

    fn insert_tokens(&self, tokens: Vec<Token>) -> Result<()> {
        for token in tokens.iter() {
            self.conn.execute(
                r#"INSERT OR IGNORE INTO tokens (lemma, pos) VALUES (?1, ?2)"#,
                params![token.lemma, token.pos.to_int()],
            )?;
        }

        Ok(())
    }

    fn insert_history_token(&self, history_token: &HistoryToken) -> Result<()> {
        self.conn.execute(
            r#"
            INSERT OR IGNORE INTO historytokens (history_id, token_id, total_unknown, total_seen)
                VALUES (?1, ?2, ?3, ?4)"#,
            params![
                history_token.history_id,
                history_token.token_id,
                history_token.total_unknown,
                history_token.total_seen
            ],
        )?;

        Ok(())
    }

    pub fn insert_state(&self, state: &State) -> Result<()> {
        let position = state.position.as_ref().ok_or_else(|| anyhow!("[error] Position cannot be None"))?;
        let action = state.action.as_ref().map(|action| action.number());

        self.conn.execute(
            r#"INSERT OR IGNORE INTO state (file_id, idx, line, operation_num, action)
                VALUES (?1, ?2, ?3, ?4, ?5)
                ON CONFLICT(file_id, operation_num) DO UPDATE SET action=?5"#,
                params![state.file_id, position.index, position.line, state.operation_num, action])?;

        Ok(())
    }
}
