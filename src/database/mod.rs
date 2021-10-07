/// The database has the following main tables:
///
/// * `files`
/// * `tokens`
/// * `state`
/// * `history`
/// * `history-tokens`
/// * `file-tokens`
///
///
///
/// The `files` table contains information on each imported file.
///
/// * `eof` - Whether the file latest state is at the end of the file
/// * `id` - A unique integer that represents the file in the database
/// * `name` - The name of the file
///
///
///
/// The `tokens` table contains information on every token the user has ever seen.
///
/// * `id` - A unique integer that represents the token in the database
/// * `lemma` - The lemma of the word
/// * `pos` - The part of speech of the word
///
///
///
/// The `state` table contains information on the current reading state of each imported file.
///
/// * `file_id` - The identifier of the file the state belongs to
/// * `id` - A unique integer that represents the given state in the database
/// * `idx` - The index of the word in the current line that is visible in the reader
/// * `line` - The current line that is visible in the reader
/// * `operation` - An integer representing the order of the states
/// * `total` - The total number of words seen by the user (a word may consist of more than one token)
/// * `unknown` - The total number of words marked as unknown by the user (a word may consist of more than one token)
/// * `action` - The action the user input when seeing the current word either `known` or `unknown`
///
///
///
/// The `history` table contains information on each previous read through of each imported file.
///
/// * `date` - The date the read through was completed
/// * `file_id` - The identifier of the file the state belongs to
/// * `id` - A unique integer that represents the given history in the database
/// * `total` - The total number of words seen by the user (a word may consist of more than one token)
/// * `unknown` - The total number of words marked as unknown by the user (a word may consist of more than one token)
///
///
///
/// The `history-tokens` table is a join table between the `history` and `tokens` tables.
/// It contains the tokens for each history.
///
/// * `history_id` - The id of the history the token belongs to
/// * `token_id` - The id of the token the history references
/// * `unknown` - The total number of times a token was marked as unknown by the user during a given read through
/// * `total` - The total number of times the token was scene by the user during a given read through
use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;

mod create;
mod insert;
mod select;

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
}
