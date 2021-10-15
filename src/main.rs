use anyhow::Result;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use app::launch_app;
use database::word;
use database::{
    Database, Filter, Operation, ReaderState, Sort, StatisticsState, Status, TokenInfo, TokenState,
    Word,
};

pub use application_state::{ApplicationState, View};

pub mod app;
pub mod application_state;
pub mod database;

pub fn main() -> Result<()> {
    let resources = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");

    let share_dir = resources.join("reader");

    let database = Rc::new(RefCell::new(Database::new(&share_dir)?));
    let reader_state = database.borrow_mut().current()?;
    let statistics_state = StatisticsState::empty();
    let token_state = TokenState::empty();

    let initial_state = ApplicationState {
        reader_state,
        statistics_state,
        token_state,

        current_view: View::Reader,

        database,
    };

    launch_app(initial_state)?;

    Ok(())
}
