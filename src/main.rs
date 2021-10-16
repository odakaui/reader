use anyhow::Result;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use app::launch_app;
use database::word;
use database::{
    Database, File, FileState, Filter, Operation, ReaderState, Sort, StatisticsState, Status,
    Token, TokenInfo, TokenState, Word,
};

pub use application_state::{ApplicationState, View};

pub mod app;
pub mod application_state;
pub mod database;

#[cfg(debug_assertions)]
fn share_dir() -> PathBuf {
    let resources = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");

    resources.join("reader")
}

#[cfg(not(debug_assertions))]
fn share_dir() -> PathBuf {
    let share = dirs::home_dir()
        .expect("failed to open home directory")
        .join(".local/share");

    share.join("reader")
}

pub fn main() -> Result<()> {
    let share_dir = share_dir();

    let database = Rc::new(RefCell::new(Database::new(&share_dir)?));
    let reader_state = database.borrow_mut().current()?;
    let statistics_state = StatisticsState::empty();
    let token_state = TokenState::empty();
    let file_state = FileState::empty();

    let initial_state = ApplicationState {
        reader_state,
        statistics_state,
        token_state,
        file_state,

        current_view: View::Reader,

        database,
    };

    launch_app(initial_state)?;

    Ok(())
}
