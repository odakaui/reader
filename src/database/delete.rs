use super::{Database, HistoryToken};
use crate::{History, Token};
use anyhow::Result;
use rusqlite::params;

impl Database {
    pub fn delete_tokens_for_history(
        &self,
        history: &History,
        tokens: Vec<Token>,
        is_unknown: bool,
    ) -> Result<()> {
        let history_id = history.id;

        for token in tokens.iter() {
            let token_id = self.select_id_for_token(token)?;

            match self.select_history_token_for_history_id_and_token_id(history_id, token_id) {
                Ok(mut history_token) => {
                    history_token.total_seen -= 1;

                    if is_unknown {
                        history_token.total_unknown -= 1;
                    }

                    if history_token.total_seen < 1 {
                        self.delete_history_token(&history_token)?;
                    } else {
                        self.update_history_token(&history_token)?;
                    }
                }
                Err(e) => {
                    println!("[error] The history_token does not exist.");
                }
            }
        }

        Ok(())
    }

    fn delete_history_token(&self, history_token: &HistoryToken) -> Result<()> {
        self.conn.execute(
            r#"DELETE FROM historytokens WHERE history_id=? AND token_id=?"#,
            params![history_token.history_id, history_token.token_id],
        )?;

        Ok(())
    }
}
