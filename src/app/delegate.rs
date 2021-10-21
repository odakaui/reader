use super::{COPY, FILES, MARK_KNOWN, LEARNED, MARK_UNKNOWN, OPEN, READER, REDO, STATISTICS, TOKENS, UNDO};
use crate::{ApplicationState, Filter, Operation, View};
use anyhow::{anyhow, Result};
use druid::{
    commands, AppDelegate, Application, Clipboard, Command, DelegateCtx, Env, Handled, Target,
};
use std::{fs, io::BufReader, io::Read, path::Path};

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

            self.next(data, Operation::MarkUnknown)
                .expect("Mark Unknown failed.");

            return Handled::Yes;
        } else if cmd.is(MARK_KNOWN) {
            println!("Mark Known");

            self.next(data, Operation::MarkKnown)
                .expect("Mark Known failed.");

            return Handled::Yes;
        } else if cmd.is(UNDO) {
            println!("Undo");

            self.undo(data).expect("[error] Undo failed.");

            return Handled::Yes;
        } else if cmd.is(REDO) {
            println!("Redo");

            self.redo(data).expect("[error] Redo failed.");

            return Handled::Yes;
        } else if cmd.is(READER) {
            println!("Reader");

            self.reader(data).expect("[error] Reader failed.");

            return Handled::Yes;
        } else if cmd.is(STATISTICS) {
            println!("Statistics");

            self.statistics(data).expect("[error] Statistics failed.");

            return Handled::Yes;
        } else if cmd.is(TOKENS) {
            println!("Tokens");

            self.tokens(data).expect("tokens failed.");

            return Handled::Yes;
        } else if cmd.is(FILES) {
            println!("Files");

            self.files(data).expect("files failed.");

            return Handled::Yes;
        } else if cmd.is(OPEN) {
            println!("Open");

            self.open(data, *cmd.get(OPEN).unwrap())
                .expect("open failed.");

            return Handled::Yes;
        } else if cmd.is(COPY) {
            println!("Copy");

            let mut clipboard = Application::global().clipboard();
            clipboard.put_string(cmd.get(COPY).unwrap());
        } else if cmd.is(LEARNED) {
            println!("Learned");

            let database = data.database.borrow_mut();

            let id = *cmd.get(LEARNED).expect("Failed to unwrap token id");
            let filter = &data.token_state.filter;

            let token_state = database.update_learned(id, filter).expect("Failed to update token id");

            data.token_state = token_state;
        }

        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            self.import(data, file_info.path())
                .expect("[error] Open File failed.");
        }

        Handled::No
    }
}

impl Delegate {
    fn next(&self, data: &mut ApplicationState, action: Operation) -> Result<()> {
        let database = data.database.borrow_mut();

        data.reader_state = database.next(&action)?;

        Ok(())
    }

    fn undo(&self, data: &mut ApplicationState) -> Result<()> {
        let database = data.database.borrow_mut();

        data.reader_state = database.undo()?;

        Ok(())
    }

    fn redo(&self, data: &mut ApplicationState) -> Result<()> {
        let database = data.database.borrow_mut();

        data.reader_state = database.redo()?;

        Ok(())
    }

    fn import(&self, data: &mut ApplicationState, path: &Path) -> Result<()> {
        let mut database = data.database.borrow_mut();

        data.reader_state = database.import(&path.to_path_buf())?;

        Ok(())
    }

    fn reader(&self, data: &mut ApplicationState) -> Result<()> {
        data.current_view = View::Reader;

        Ok(())
    }

    fn statistics(&self, data: &mut ApplicationState) -> Result<()> {
        let database = data.database.borrow_mut();
        data.statistics_state = database.statistics()?;

        data.current_view = View::Statistics;

        Ok(())
    }

    fn files(&self, data: &mut ApplicationState) -> Result<()> {
        let database = data.database.borrow_mut();
        data.file_state = database.files()?;

        data.current_view = View::Files;

        Ok(())
    }

    fn tokens(&self, data: &mut ApplicationState) -> Result<()> {
        let database = data.database.borrow_mut();
        data.token_state = database.tokens(&Filter::All)?;

        data.current_view = View::Tokens;

        Ok(())
    }

    fn open(&self, data: &mut ApplicationState, id: i32) -> Result<()> {
        let mut database = data.database.borrow_mut();

        data.reader_state = database.open(id)?;

        data.current_view = View::Reader;

        Ok(())
    }
}
