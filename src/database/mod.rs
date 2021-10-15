use anyhow::{anyhow, Result};
use rusqlite::Connection;
use std::path::{Path, PathBuf};

use file::File;
use state::{Position, State};
use token::{Token, POS};
use tokenizer::Tokenizer;

pub use file::word;
pub use file::Word;
pub use reader_state::{Status, ReaderState};
pub use state::Operation;

mod common;
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
            return Ok(ReaderState::empty())    
        }

        let conn = &self.conn;
        let file = self.file()?;

        let state = file::reset(conn, &file)?;

        let reader_state = ReaderState::new(&file, &state);

        Ok(reader_state)
    }

    pub fn current(&mut self) -> Result<ReaderState> {
        if self.file.is_none() {
            return Ok(ReaderState::empty())    
        }

        let conn = &self.conn;
        let file = self.file()?;

        let state = file::current_state(conn, &file)?;

        let reader_state = ReaderState::new(&file, &state);
        Ok(reader_state)
    }

    pub fn next(&self, action: &Operation) -> Result<ReaderState> {
        if self.file.is_none() {
            return Ok(ReaderState::empty())    
        }

        let conn = &self.conn;
        let file = self.file()?;

        let state = file::next(conn, &file, action)?;

        let reader_state = ReaderState::new(&file, &state);
        Ok(reader_state)
    }

    pub fn undo(&self) -> Result<ReaderState> {
        if self.file.is_none() {
            return Ok(ReaderState::empty())    
        }

        let conn = &self.conn;
        let file = self.file()?;

        let state = file::undo(conn, &file)?;

        let reader_state = ReaderState::new(&file, &state);
        Ok(reader_state)
    }

    pub fn redo(&self) -> Result<ReaderState> {
        if self.file.is_none() {
            return Ok(ReaderState::empty())    
        }

        let conn = &self.conn;
        let file = self.file()?;

        let state = file::redo(conn, &file)?;

        let reader_state = ReaderState::new(&file, &state);
        Ok(reader_state)
    }

    fn file(&self) -> Result<File> {
        Ok(self
            .file.as_ref()
            .ok_or_else(|| anyhow!("There is no currently open file."))?.clone())
    }

    // pub fn undo(&self, state: &State) -> Result<State> {
    //     unimplemented!()
    // }

    // pub fn redo(&self, state: &State) -> Result<State> {
    //     unimplemented!()
    // }

    // pub fn files(&self) -> Result<Vec<File>> {
    //     unimplemented!()
    // }

    // pub fn history(&self, file: &File) -> Result<Vec<History>> {
    //     unimplemented!()
    // }

    // pub fn statistics(&self, history: &History) -> Result<Statistics> {
    //     unimplemented!()
    // }

    // pub fn unknown(&self) -> Result<Vec<TokenInfo>> {
    //     unimplemented!()
    // }

    // pub fn toggle_learned(&self, token: &Token) -> Result<bool> {
    //     unimplemented!()
    // }
}
