use super::{Database, HistoryToken};
use anyhow::Result;
use rusqlite::params;

impl Database {
    pub fn update_history_token(&self, history_token: &HistoryToken) -> Result<()> {
        self.conn.execute(
            r#"UPDATE historytokens SET total_unknown=?1, total_seen=?2
                WHERE history_id=?3 AND token_id=?4"#,
            params![
                history_token.total_unknown,
                history_token.total_seen,
                history_token.history_id,
                history_token.token_id
            ],
        )?;

        Ok(())
    }
}
