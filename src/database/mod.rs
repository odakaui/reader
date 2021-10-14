use anyhow::{anyhow, Result};
use rusqlite::{params, Connection};
use std::{path, fs};
use serde::{Deserialize, Serialize};


pub use file::File;
pub use token::Token;
pub use state::State;

pub mod file;
pub mod history;
pub mod history_token;
pub mod state;
pub mod token;

mod common;
mod current_file;
mod current_history;
mod current_state;

pub struct Database {
    files_path: path::PathBuf,
    conn: Connection,
}

impl Database {
    pub fn new(path: &path::PathBuf) -> Result<Self> {
        // create the database connection and the files directory
        let database_path = path.join("reader.db");
        let files_path = path.join("files");

        // create files_dir
        common::create_dir(&files_path)?;

        // initialize the database
        let conn = Connection::open(database_path)?;
        file::initialize(&conn)?;
        history::initialize(&conn)?;
        state::initialize(&conn)?;
        token::initialize(&conn)?;
        history_token::initialize(&conn)?;
        current_file::initialize(&conn)?;
        current_history::initialize(&conn)?;
        current_state::initialize(&conn)?;

        let database = Database {
            files_path,
            conn,
        };

        Ok(database)
    }

    // // add a file to the database and return a state object representing the initial state
    // pub fn import(&mut self, path: &path::PathBuf) -> Result<ReaderState> {
    //     unimplemented!()
    // }

    // // open a file and return a state object representing the current state
    // pub fn open(&self, file: &File) -> Result<State> {
    //     unimplemented!()
    // }

    // // save a File to a text file
    // pub fn save_file(&self, file: &File) -> Result<()> {
    //     unimplemented!()
    // }

    // // load a File from a text file
    // pub fn load_file(&self) -> Result<()> {
    //     unimplemented!()
    // }

    // // clear all state from current History and create a new History with an initial State
    // pub fn reset(&self, state: &State) -> Result<State> {
    //     unimplemented!()
    // }

    // // insert the current State into the database
    // pub fn current(&self, state: &State) -> Result<()> {
    //     unimplemented!()
    // }

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
