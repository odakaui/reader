use crate::Token;
use anyhow::{anyhow, Result};
pub use file::File;
pub use history::History;
pub use history_token::HistoryToken;
use rusqlite::Connection;
pub use state::State;
use std::path;

mod delete;
mod file;
mod history;
mod history_token;
mod insert;
mod select;
mod state;
mod table;
mod update;

pub struct Statistics {}

pub struct Database {
    conn: Connection,
    ron_path: Box<path::PathBuf>,
}

impl Database {
    pub fn new(
        path: &path::Path,
        ron_path: &path::PathBuf,
    ) -> Result<Self> {
        // create the database
        let conn = Connection::open(path)?;
        let database = Database {
            conn,
            ron_path: Box::new(ron_path.clone()),
        };

        table::create_files(&conn)?;
        table::create_history(&conn)?;
        table::create_history_tokens(&conn)?;
        table::create_state(&conn)?;
        table::create_tokens(&conn)?;

        Ok(database)
    }

    // add a file to the database and return a state object representing the initial state
    pub fn import(&self, name: String) -> Result<State> {
        // throw error if file had already been imported
        if file::select_file(&self.conn, &name).is_ok() {
            println!("{} has already been imported.", &name);

            return Err(anyhow!("{} has already been imported.", &name));
        }

        // insert the file into the database
        file::insert_file(&self.conn, &name)?;
        let file = file::select_file(&self.conn, &name)?;

        // create history for the file
        history::insert_history(&self.conn, file.id)?;
        let history = history::select_current_history(&self.conn, file.id)?;

        // create state for the file
        state::insert_initial_state(&self.conn, history.id)?;
        let state = state::select_current_state(&self.conn, history.id)?;

        // set current_state for History
        history::update_history_with_current_operation(
            &self.conn,
            history.id,
            state.operation_num,
        )?;

        Ok(state)
    }

    // open a file and return a state object representing the current state
    pub fn open(&self, file: &File) -> Result<State> {
        // throw error if file has not been imported
        if file::select_file(&self.conn, &file.name).is_err() {
            println!("{} has not been imported.", file.name);

            return Err(anyhow!("{} has not been imported.", file.name));
        }

        // get the current state for file
        let history = history::select_current_history(&self.conn, file.id)?;
        let state = state::select_current_state(&self.conn, history.id)?;

        Ok(state)
    }

    // save a File to a text file
    pub fn save_file(&self, file: &File) -> Result<()> {
        file::save_file(&self.ron_path, file)?;

        Ok(())
    }

    // load a File from a text file
    pub fn load_file(&self) -> Result<()> {
        file::load_file(&self.ron_path)?;

        Ok(())
    }

    // clear all state from current History and create a new History with an initial State
    pub fn reset(&self, state: &State) -> Result<State> {
        let history = history::select_history_for_id(&self.conn, state.history_id)?;

        // clear all states for history
        state::delete_state(&self.conn, history.id)?;
        history::update_history_with_current_operation(&self.conn, history.id, 0)?;

        // create a new history
        history::insert_history(&self.conn, history.file_id)?;

        // create the initial state for history
        let file_id = history.file_id;
        let new_history = history::select_current_history(&self.conn, file_id)?;
        state::insert_initial_state(&self.conn, new_history.id)?;

        // update current_operation
        let state = state::select_current_state(&self.conn, new_history.id)?;
        history::update_history_with_current_operation(
            &self.conn,
            new_history.id,
            state.operation_num,
        )?;

        Ok(state)
    }

    // insert the current State into the database
    pub fn current(&self, state: &State) -> Result<()> {
        // clear the redo queue
        let history = history::select_history_for_id(&self.conn, state.history_id)?;
        state::delete_state_operation_num(&self.conn, state.history_id, history.current_operation)?;

        // insert state
        state::insert_state(&self.conn, state)?;

        // update current_operation
        let state = state::select_current_state(&self.conn, history.id)?;
        history::update_history_with_current_operation(
            &self.conn,
            history.id,
            state.operation_num,
        )?;

        Ok(())
    }

    pub fn undo(&self, state: &State) -> Result<State> {
        // raise error if operation_num is 1
        if state.operation_num <= 1 {
            println!("operation_num must be greater than 1.");

            return Err(anyhow!("operation_num must be greater than 1"));
        }

        // get the new state and update History
        let new_state = state::select_state_for_operation_num(
            &self.conn,
            state.history_id,
            state.operation_num - 1,
        )?;
        history::update_history_with_current_operation(
            &self.conn,
            new_state.history_id,
            new_state.operation_num,
        )?;

        Ok(new_state)
    }

    pub fn redo(&self, state: &State) -> Result<State> {
        // get the next state or raise error if redo queue is empty
        let new_state = state::select_state_for_operation_num(
            &self.conn,
            state.history_id,
            state.operation_num + 1,
        )
        .or_else(|_| return Err(anyhow!("redo queue is empty")))?;

        // update operation_num
        history::update_history_with_current_operation(
            &self.conn,
            new_state.history_id,
            new_state.operation_num,
        )?;

        Ok(new_state)
    }

    pub fn files(&self) -> Result<Vec<File>> {
        unimplemented!()
    }

    pub fn history(&self, file: &File) -> Result<Vec<History>> {
        unimplemented!()
    }

    pub fn statistics(&self, history: &History) -> Result<Statistics> {
        unimplemented!()
    }

    pub fn unknown(&self) -> Result<Vec<Token>> {
        unimplemented!()
    }

    pub fn toggle_learned(&self, token: &Token) -> Result<bool> {
        unimplemented!()
    }

    pub fn add_tokens_known(&self, history: &History, tokens: Vec<Token>) -> Result<()> {
        self.insert_tokens_for_history(history, tokens, false)?;

        Ok(())
    }

    pub fn add_tokens_unknown(&self, history: &History, tokens: Vec<Token>) -> Result<()> {
        self.insert_tokens_for_history(history, tokens, true)?;

        Ok(())
    }

    pub fn remove_tokens_known(&self, history: &History, tokens: Vec<Token>) -> Result<()> {
        self.delete_tokens_for_history(history, tokens, false)?;

        Ok(())
    }

    pub fn remove_tokens_unknown(&self, history: &History, tokens: Vec<Token>) -> Result<()> {
        self.delete_tokens_for_history(history, tokens, true)?;

        Ok(())
    }
}
