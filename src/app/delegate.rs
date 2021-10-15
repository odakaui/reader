use super::{MARK_KNOWN, MARK_UNKNOWN, READER, REDO, STATISTICS, UNDO};
use crate::{ApplicationState, Operation, View};
use anyhow::{anyhow, Result};
use druid::{commands, AppDelegate, Command, DelegateCtx, Env, Handled, Target};
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
}
