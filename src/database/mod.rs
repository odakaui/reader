use anyhow::{anyhow, Result};
use rusqlite::Connection;
use std::path::{Path, PathBuf};

use file::File;
use state::{Position, State};
use token::{Token, POS};
use tokenizer::Tokenizer;

pub use database_error::DatabaseError;
pub use file::word;
pub use file::Word;
pub use reader_state::{ReaderState, Status};
pub use state::Operation;

mod common;
mod database_error;
mod file;
mod history;
mod history_token;
mod reader_state;
mod state;
mod token;
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

        let file = file::insert_file(conn, &source_file.to_path_buf(), target_dir)?;
        let state = file::current_state(conn, &file)?;

        let reader_state = ReaderState::new(&file, &state);

        self.file = Some(file);

        Ok(reader_state)
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
                    println!("eof");

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
                    println!("undo stack is empty.");

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
                    println!("redo stack is empty.");

                    Ok(get_current(conn, &file)?)
                } else {
                    Err(e)
                }
            }
        }
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
    e.downcast_ref::<DatabaseError>().map_or(false, |e| {
        e == database_error
    })
}
