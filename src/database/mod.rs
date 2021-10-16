use anyhow::{anyhow, Result};
use rusqlite::Connection;
use std::path::{Path, PathBuf};

use state::{Position, State};
use tokenizer::Tokenizer;

pub use database_error::DatabaseError;
pub use file::word;
pub use file::File;
pub use file::Word;
pub use file_state::FileState;
pub use history_token::HistoryToken;
pub use reader_state::{ReaderState, Status};
pub use state::Operation;
pub use statistics_state::StatisticsState;
pub use token::{Token, POS};
pub use token_info::TokenInfo;
pub use token_state::{Filter, Sort, TokenState};

mod common;
mod database_error;
mod file;
mod file_state;
mod history;
mod history_token;
mod reader_state;
mod state;
mod statistics_state;
mod token;
mod token_info;
mod token_state;
mod tokenizer;

pub struct Database {
    files_dir: PathBuf,
    conn: Connection,
    file: Option<File>,
}

impl Database {
    pub fn new(path: &Path) -> Result<Self> {
        let files_dir = path.join("files");
        common::create_dir(&files_dir)?;

        let database_path = path.join("reader.db");
        let conn = Connection::open(database_path)?;

        file::initialize(&conn, &files_dir)?;

        let file = file::current_file(&conn, &files_dir).ok();

        let database = Database {
            files_dir,
            conn,
            file,
        };

        Ok(database)
    }

    pub fn import(&mut self, source_file: &Path) -> Result<ReaderState> {
        let target_dir = &self.files_dir;
        let conn = &self.conn;

        match file::insert_file(conn, &source_file.to_path_buf(), target_dir) {
            Ok(file) => {
                self.file = Some(file.clone());

                let state = file::current_state(conn, &file)?;

                Ok(ReaderState::new(&file, &state))
            }
            Err(e) => {
                if is_error(&e, &DatabaseError::FileExists) {
                    println!("{}", e);

                    Ok(get_current(conn, &get_file(&self.file)?)?)
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn open(&mut self, id: i32) -> Result<ReaderState> {
        let target_dir = &self.files_dir;
        let conn = &self.conn;

        let file = file::select_file(conn, target_dir, id)?;
        let state = file::current_state(conn, &file)?;

        let reader_state = ReaderState::new(&file, &state);

        self.file = Some(file);

        Ok(reader_state)
    }

    pub fn reset(&self) -> Result<ReaderState> {
        if self.file.is_none() {
            return Ok(ReaderState::empty());
        }

        let conn = &self.conn;
        let file = get_file(&self.file)?;

        let state = file::reset(conn, &file)?;

        let reader_state = ReaderState::new(&file, &state);

        Ok(reader_state)
    }

    pub fn current(&mut self) -> Result<ReaderState> {
        if self.file.is_none() {
            return Ok(ReaderState::empty());
        }

        let conn = &self.conn;
        let file = get_file(&self.file)?;

        let state = file::current_state(conn, &file)?;

        let reader_state = ReaderState::new(&file, &state);
        Ok(reader_state)
    }

    pub fn next(&self, action: &Operation) -> Result<ReaderState> {
        if self.file.is_none() {
            return Ok(ReaderState::empty());
        }

        let conn = &self.conn;
        let file = get_file(&self.file)?;

        match file::next(conn, &file, action) {
            Ok(state) => {
                let reader_state = ReaderState::new(&file, &state);
                Ok(reader_state)
            }
            Err(e) => {
                if is_error(&e, &DatabaseError::Eof) {
                    println!("{}", e);

                    Ok(get_current(conn, &file)?)
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn undo(&self) -> Result<ReaderState> {
        if self.file.is_none() {
            return Ok(ReaderState::empty());
        }

        let conn = &self.conn;
        let file = get_file(&self.file)?;

        match file::undo(conn, &file) {
            Ok(state) => {
                let reader_state = ReaderState::new(&file, &state);

                Ok(reader_state)
            }
            Err(e) => {
                if is_error(&e, &DatabaseError::UndoEmpty) {
                    println!("{}", e);

                    Ok(get_current(conn, &file)?)
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn redo(&self) -> Result<ReaderState> {
        if self.file.is_none() {
            return Ok(ReaderState::empty());
        }

        let conn = &self.conn;
        let file = get_file(&self.file)?;

        match file::redo(conn, &file) {
            Ok(state) => {
                let reader_state = ReaderState::new(&file, &state);

                Ok(reader_state)
            }
            Err(e) => {
                if is_error(&e, &DatabaseError::RedoEmpty) {
                    println!("{}", e);

                    Ok(get_current(conn, &file)?)
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn statistics(&self) -> Result<StatisticsState> {
        let conn = &self.conn;
        let file = get_file(&self.file)?;

        file::statistics(conn, &file)
    }

    pub fn tokens(&self, filter: &Filter) -> Result<TokenState> {
        let conn = &self.conn;

        file::tokens(conn, filter)
    }

    pub fn save(&self, tokens: &Vec<TokenInfo>, filter: &Filter) -> Result<TokenState> {
        let conn = &self.conn;

        TokenInfo::save(conn, tokens)?;

        file::tokens(conn, filter)
    }

    pub fn files(&self) -> Result<FileState> {
        let conn = &self.conn;

        file::files(conn)
    }
}

fn get_file(file: &Option<File>) -> Result<File> {
    Ok(file.as_ref().ok_or(DatabaseError::FileOpen)?.clone())
}

fn get_current(conn: &Connection, file: &File) -> Result<ReaderState> {
    let state = file::current_state(conn, file)?;

    let reader_state = ReaderState::new(file, &state);
    Ok(reader_state)
}

fn is_error(e: &anyhow::Error, database_error: &DatabaseError) -> bool {
    e.downcast_ref::<DatabaseError>()
        .map_or(false, |e| e == database_error)
}
