use crate::{History, Token};
use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;

pub use history_token::HistoryToken;

mod create;
mod history_token;
mod insert;
mod select;
mod update;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &Path) -> Result<Self> {
        let db = Database {
            conn: Connection::open(path)?,
        };

        db.initialize()?;

        Ok(db)
    }

    pub fn add_tokens_known(&self, history: &History, tokens: Vec<Token>) -> Result<()> {
        self.insert_tokens_for_history(history, tokens, false)?;

        Ok(())
    }

    pub fn add_tokens_unknown(&self, history: &History, tokens: Vec<Token>) -> Result<()> {
        self.insert_tokens_for_history(history, tokens, true)?;

        Ok(())
    }
}
