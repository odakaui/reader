use super::{history_token, token};
use super::{HistoryToken, Token};
use anyhow::Result;
use rusqlite::Connection;
use druid::{Data, Lens};

#[derive(Clone, Debug, PartialEq, Data, Lens)]
pub struct TokenInfo {
    token: Token,
    history_token: HistoryToken,
}

impl TokenInfo {
    pub fn new(token: Token, history_token: HistoryToken) -> Self {
        TokenInfo {
            token,
            history_token,
        }
    }

    pub fn to_token_info(conn: &Connection, history_id: i32) -> Result<Vec<TokenInfo>> {
        let history_tokens = history_token::select_history_tokens(conn, history_id)?;

        Ok(history_tokens
            .iter()
            .map(|history_token| -> Result<TokenInfo> {
                let id = history_token.token_id;
                let token: Token = token::select_token(conn, id)?;

                Ok(TokenInfo::new(token, history_token.clone()))
            })
            .flatten()
            .collect())
    }

    pub fn total_seen(&self) -> i32 {
        self.history_token.total_seen
    }

    pub fn total_unknown(&self) -> i32 {
        self.history_token.total_unknown
    }

    pub fn percent_known(&self) -> i32 {
        let total_seen = self.history_token.total_seen as f64;
        let total_known = total_seen - self.history_token.total_unknown as f64;

        (total_known / total_seen * 100.) as i32
    }

    pub fn lemma(&self) -> String {
        self.token.lemma.to_string()
    }
}
