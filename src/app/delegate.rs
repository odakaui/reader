use super::{MARK_KNOWN, MARK_UNKNOWN, REDO, UNDO};
use crate::{compressor, reader, ApplicationState, Operation, ReaderState, State};
use anyhow::{anyhow, Result};
use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target};
use std::{fs, path::Path};

pub struct Delegate;

impl AppDelegate<ApplicationState> for Delegate {
    fn command(
        &mut self,
        _: &mut DelegateCtx<'_>,
        _: Target,
        cmd: &Command,
        data: &mut ApplicationState,
        _: &Env,
    ) -> Handled {
        if cmd.is(MARK_UNKNOWN) {
            println!("Mark Unknown");

            self.add_tokens(data, Operation::MarkUnknown)
                .expect("Mark Unknown failed.");

            Handled::Yes
        } else if cmd.is(MARK_KNOWN) {
            println!("Mark Known");

            self.add_tokens(data, Operation::MarkKnown)
                .expect("Mark Known failed.");

            Handled::Yes
        } else if cmd.is(UNDO) {
            println!("Undo");

            self.undo(data).expect("[error] Undo failed.");

            Handled::Yes
        } else if cmd.is(REDO) {
            println!("Redo");

            self.redo(data).expect("[error] Redo failed.");

            Handled::Yes
        } else {
            Handled::No
        }
    }
}

impl Delegate {
    fn add_tokens(&self, data: &mut ApplicationState, action: Operation) -> Result<()> {
        // return if reader_state is None
        if data.reader_state.is_none() {
            println!("[error] File not loaded.");
        }

        let reader_state = data.reader_state.as_mut().unwrap();

        let article = &reader_state.article;
        let history = &reader_state.history;

        let mut current_state = reader_state.current_state.clone();

        // return if current_position is None
        if current_state.position.is_none() {
            println!("[info] EOF reached.");

            return Ok(());
        }

        // add the tokens to the reader_statebase
        let database = &data.database.borrow_mut();
        let word = compressor::compress(article, &current_state);

        match action {
            Operation::MarkKnown => database.add_tokens_known(history, word.tokens)?,
            Operation::MarkUnknown => database.add_tokens_unknown(history, word.tokens)?,
        }

        // set the action of the current_state and move it to the undo_stack
        current_state.action = Some(action);
        reader_state.undo_stack.push(current_state.clone());

        // set the current_state
        let next_position = reader::next_position(article, &current_state);
        let file_id = article.id;
        let next_operation_num = current_state.operation_num + 1;

        reader_state.current_state = State {
            file_id,
            position: next_position,
            operation_num: next_operation_num,
            action: None,
        };

        // clear the redo stack
        reader_state.redo_stack.clear();

        Ok(())
    }

    fn undo(&self, data: &mut ApplicationState) -> Result<()> {
        // return if reader_state is None
        if data.reader_state.is_none() {
            println!("[error] File not loaded.");

            return Ok(());
        }

        let reader_state = data.reader_state.as_mut().unwrap();

        // return if the undo_stack is empty
        if reader_state.undo_stack.is_empty() {
            println!("[warning] The undo stack is empty.");

            return Ok(());
        }

        // remove the tokens from the database
        let previous_state = reader_state
            .undo_stack
            .pop()
            .expect("Failed to unwrap undo_stack");
        let history = &reader_state.history;
        let article = &reader_state.article;

        let database = data.database.borrow_mut();
        let word = compressor::compress(article, &previous_state);

        match previous_state
            .action
            .as_ref()
            .expect("[error] Failed to unwrap action.")
        {
            Operation::MarkKnown => {
                database.remove_tokens_known(history, word.tokens)?;
            }
            Operation::MarkUnknown => {
                database.remove_tokens_unknown(history, word.tokens)?;
            }
        }

        // add the current state to the redo_stack
        let current_state = reader_state.current_state.clone();

        reader_state.redo_stack.push(current_state);

        // set the current_state to the previous_state
        reader_state.current_state = previous_state;

        Ok(())
    }

    fn redo(&self, data: &mut ApplicationState) -> Result<()> {
        // return if reader_state is None
        if data.reader_state.is_none() {
            println!("[error] File not loaded.");

            return Ok(());
        }

        let reader_state = data.reader_state.as_mut().unwrap();

        // return if the redo_state is empty
        if reader_state.redo_stack.is_empty() {
            println!("[warning] The redo stack is empty.");

            return Ok(());
        }

        // add the tokens to the database
        let current_state = reader_state.current_state.clone();
        let history = &reader_state.history;
        let article = &reader_state.article;

        let database = data.database.borrow_mut();
        let word = compressor::compress(article, &current_state);

        match current_state.action.as_ref().unwrap() {
            Operation::MarkKnown => database.add_tokens_known(history, word.tokens)?,
            Operation::MarkUnknown => database.add_tokens_unknown(history, word.tokens)?,
        }

        // set the current_state to the next_state
        let next_state = reader_state
            .redo_stack
            .pop()
            .expect("[error] Failed to unwrap redo_stack.");

        reader_state.current_state = next_state;

        // add the current_state ot the undo_stack
        reader_state.undo_stack.push(current_state);

        Ok(())
    }

    fn create_dir(path: &Path) -> Result<()> {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }

        Ok(())
    }

    fn clean(text: &str) -> Vec<String> {
        let lines = text.lines();

        lines
            .map(|x| x.chars().filter(|c| !c.is_whitespace()).collect())
            .filter(|x: &String| !x.is_empty())
            .collect()
    }

    fn file_stem(path: &Path) -> Result<String> {
        Ok(path
            .file_stem()
            .ok_or(anyhow!("Failed to parse file name."))?
            .to_str()
            .ok_or(anyhow!("Failed to convert file name."))?
            .to_string())
    }

    fn file_name(path: &Path) -> Result<String> {
        Ok(path
            .file_name()
            .ok_or(anyhow!("Failed to parse file name."))?
            .to_str()
            .ok_or(anyhow!("Failed to convert file name."))?
            .to_string())
    }

    fn import(&self, data: &mut ApplicationState) -> Result<()> {
        // create files_dir
        let files_dir = &data.files_dir;
        Self::create_dir(&files_dir)?;

        Ok(())

        // // add the file to the database
        // let name = Self::file_name(file)?;

        // db.insert_file(&name)?;
        // let f = &db.select_file_for_name(&name)?;

        // // add the file to the file folder
        // let contents = read_file(file)?;
        // let clean_lines = clean(&contents);
        // let import_path = import_dir.join(&name);
        // let mut tokenizer = tokenizer::new()?;

        // let mut tokenized_lines: vec<line> = vec::new();
        // for x in clean_lines.iter() {
        //     let tokens = tokenizer.tokenize(x)?;
        //     let line = line {
        //         sentence: x.into(),
        //         tokens,
        //     };

        //     tokenized_lines.push(line);
        // }

        // let article = article::new(f.id, &name, &tokenized_lines);

        // fs::write(import_path, ron::to_string(&article)?)?;

        // let files_dir = data.files_dir;

        // if !path.exists() {
        //     return Err(anyhow!("The file {} does not exist.", name));
        // }

        // let article = ron::from_str(&String::from_utf8(fs::read(path)?)?)?;

        // let db = database.borrow_mut();

        // // get the file id
        // let file = db.select_file_for_name(name)?;

        // // get the current history for the file
        // let history = db.select_current_history_for_file(&file)?;

        // // create the initial app state
        // let position = Position { index: 0, line: 0 };

        // // @TODO load the current state
        // let current_state = State {
        //     file_id: file.id,
        //     position: Some(position),
        //     operation_num: 0,
        //     action: None,
        // };

        // // create the ReaderState
        // let reader_state = ReaderState {
        //     current_state,
        //     redo_stack: Vec::new(),
        //     undo_stack: Vec::new(),
        //     article,
        //     history,
        // };

        // Ok(reader_state)
    }
}
