use anyhow::Result;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use app::launch_app;
use database::word;
use database::{Database, Operation, Word, Status, ReaderState};

pub use application_state::{ApplicationState, View};

pub mod app;
pub mod application_state;
pub mod database;

pub fn main() -> Result<()> {
    let resources = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");

    let share_dir = resources.join("reader");

    let database = Rc::new(RefCell::new(Database::new(&share_dir)?));
    let reader_state = database.borrow_mut().current()?;

    let initial_state = ApplicationState {
        reader_state,
        current_view: View::Reader,
        database,
    };

    launch_app(initial_state)?;

    Ok(())
}
