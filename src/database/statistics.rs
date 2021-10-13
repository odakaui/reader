use super::{file, history_token, File, History, HistoryToken};
use anyhow::Result;
use rusqlite::Connection;

pub struct Statistics {
    pub file: File,
    pub history: History,
    pub history_tokens: Vec<HistoryToken>,
}

pub fn select_statistics(conn: &Connection, history: &History) -> Result<Statistics> {
    let file = file::select_file_for_id(conn, history.file_id)?;
    let history_tokens = history_token::select_history_tokens_for_history_id(conn, history.id)?;

    Ok(Statistics {
        file,
        history: history.to_owned(),
        history_tokens,
    })
}
