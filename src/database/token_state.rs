use super::{filter, Filter, TokenInfo};
use druid::{Data, Lens};
use std::cmp::Reverse;
use std::sync::Arc;
use rusqlite::Connection;
use anyhow::Result;

#[derive(Clone, Debug, Data, Lens)]
pub struct TokenState {
    pub tokens: Arc<Vec<TokenInfo>>,
    pub filter: Filter,
}

impl TokenState {
    pub fn new(conn: &Connection) -> Result<Self> {
        let tokens = TokenInfo::all(conn)?;
        let filter = filter::get_filter(conn)?;

        let tokens = match filter {
            Filter::All => {
                let mut tokens: Vec<TokenInfo> = tokens.iter().map(|token| token.to_owned()).collect();
                tokens.sort_by_key(|token| Reverse(token.total_unknown()));

                tokens
            }
            Filter::Learned => {
                let mut tokens: Vec<TokenInfo> = tokens
                    .iter()
                    .filter(|token| token.learned())
                    .map(|token| token.to_owned())
                    .collect();
                tokens.sort_by_key(|token| Reverse(token.total_unknown()));

                tokens
            }
            Filter::Unlearned => {
                let mut tokens: Vec<TokenInfo> = tokens
                    .iter()
                    .filter(|token| !token.learned())
                    .map(|token| token.to_owned())
                    .collect();
                tokens.sort_by_key(|token| Reverse(token.total_unknown()));

                tokens
            }
        };

        Ok(TokenState {
            tokens: Arc::new(tokens),
            filter,
        })
    }

    pub fn empty() -> Self {
        let info: Vec<TokenInfo> = Vec::new();
        TokenState {
            tokens: Arc::new(info),
            filter: Filter::All,
        }
    }
}
