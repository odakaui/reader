use std::path::PathBuf;
use anyhow::{anyhow, Result};
use rusqlite::Connection;

use file::{File, Line};
use tokenizer::Tokenizer;
use state::{Position, State, Operation};
use token::{Token, POS};

pub use reader_state::ReaderState;

mod file;
mod history;
mod state;
mod reader_state;
mod token;
mod tokenizer;
mod common;
mod history_token;



pub struct Database {
    files_dir: PathBuf,
    conn: Connection,
    file: Option<File>,
}

impl Database {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let files_dir = path.join("files");
        common::create_dir(&files_dir)?;

        let database_path = path.join("reader.db");
        let conn = Connection::open(database_path)?;

        file::initialize(&conn, &files_dir)?;

        let database = Database {
            files_dir,
            conn,
            file: None,
        };

        Ok(database)
    }
    
    pub fn import(&mut self, source_file: &PathBuf) -> Result<ReaderState> {
        let target_dir = &self.files_dir;
        let conn = &self.conn;

        let file = file::insert_file(conn, source_file, target_dir)?;
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
        let conn = self.conn;
        let file = self.file()?;

        let state = file::reset(&conn, &file)?;

        let reader_state = ReaderState::new(&file, &state);

        Ok(reader_state)

    }

    pub fn current(&mut self) -> Result<ReaderState> {
        let target_dir = &self.files_dir; 
        let conn = &self.conn;
        let file = self.file()?;

        let state = file::current_state(conn, &file)?;

        let reader_state = ReaderState::new(&file, &state);
        Ok(reader_state)

    }

    pub fn next(&self, action: &Operation) -> Result<ReaderState> {
        let conn = self.conn;
        let file = self.file()?;

        let state = file::next(&conn, &file, action)?;

        let reader_state = ReaderState::new(&file, &state);
        Ok(reader_state)
    }

    pub fn undo(&self) -> Result<ReaderState> {
        let conn = self.conn;
        let file = self.file()?;

        let state = file::undo(&conn, &file)?;

        let reader_state = ReaderState::new(&file, &state);
        Ok(reader_state)
    }

    pub fn redo(&self) -> Result<ReaderState> {
        let conn = self.conn;
        let file = self.file()?;

        let state = file::redo(&conn, &file)?;

        let reader_state = ReaderState::new(&file, &state);
        Ok(reader_state)

    }

    fn file(&self) -> Result<File> {
        let conn = self.conn;
        let target_dir = &self.files_dir; 

        if self.file.is_none() {
            self.file = Some(file::current_file(&conn, target_dir)?);
        }

        Ok(self.file.ok_or(anyhow!("There is no currently open file."))?)

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
