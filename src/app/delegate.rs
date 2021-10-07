use crate::{ApplicationState, Operation, State, reader, compressor};
use druid::{AppDelegate, Target, DelegateCtx, Command, Handled, Env};
use super::{MARK_KNOWN, MARK_UNKNOWN, UNDO, REDO};
use anyhow::Result;

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
        let article = &data.article;
        let history = &data.history;

        // return if current_state is None
        if data.current_state.is_none() {
            println!("[error] current_state is None.");

            return Ok(())
        }

        let mut current_state = data
            .current_state
            .as_ref()
            .expect("Failed to unwrap current_state")
            .clone();

        // return if current_position is None
        if current_state.position.is_none() {
            println!("[info] EOF reached.");

            return Ok(());
        }

        // add the tokens to the database
        let database = &data.database.borrow_mut();
        let word = compressor::compress(article, &current_state);

        match action {
            Operation::MarkKnown => database.add_tokens_known(history, word.tokens)?,
            Operation::MarkUnknown => database.add_tokens_unknown(history, word.tokens)?,
        }

        // set the action of the current_state and move it to the undo_stack 
        current_state.action = Some(action);
        data.undo_stack.push(current_state.clone());

        // set the current_state
        let next_position = reader::next_position(article, &current_state);
        let file_id = article.id;
        let next_operation_num = current_state.operation_num + 1;

        data.current_state = Some(State {
            file_id,
            position: next_position,
            operation_num: next_operation_num,
            action: None,
        });

        // clear the redo stack
        data.redo_stack.clear();

        Ok(())
    }

    fn undo(&self, data: &mut ApplicationState) -> Result<()> {
        // return if the current_state is None or the undo stack is empty
        if data.current_state.is_none() || data.undo_stack.is_empty() {
            println!("[warning] The undo stack is empty.");

            return Ok(());
        }

        // remove the tokens from the database
        let previous_state = data.undo_stack.pop().expect("Failed to unwrap undo_stack");
        let history = &data.history;
        let article = &data.article;

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
        let current_state = data
            .current_state
            .as_ref()
            .expect("Failed to unwrap current_state").clone();

        data.redo_stack.push(current_state);

        // set the current_state to the previous_state
        data.current_state = Some(previous_state);

        Ok(())
    }

    fn redo(&self, data: &mut ApplicationState) -> Result<()> {
        // return if the current_state is None or the redo_stack is empty or current_state.action
        // is None
        if data.current_state.is_none()
            || data.redo_stack.is_empty()
            || data.current_state.as_ref().unwrap().action.is_none()
        {
            println!("[warning] The redo stack is empty.");

            return Ok(());
        }

        // add the tokens to the database
        let current_state = data.current_state.as_ref().unwrap().clone();
        let history = &data.history;
        let article = &data.article;

        let database = data.database.borrow_mut();
        let word = compressor::compress(article, &current_state);

        match current_state.action.as_ref().unwrap() {
            Operation::MarkKnown => database.add_tokens_known(history, word.tokens)?,
            Operation::MarkUnknown => database.add_tokens_unknown(history, word.tokens)?,
        }

        // set the current_state to the next_state
        let next_state = data.redo_stack.pop().expect("[error] Failed to unwrap redo_stack.");

        data.current_state = Some(next_state);

        // add the current_state ot the undo_stack
        data.undo_stack.push(current_state); 

        Ok(())
    }
}
