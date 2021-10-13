use anyhow::Result;
use app::launch_app;
use article::{Article, Line};
use database::Database;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use tokenizer::Tokenizer;

pub use application_state::{ApplicationState, ReaderState, StatisticsState, View};
pub use file::File;
pub use history::History;
pub use state::{Operation, Position, State};
pub use token::{Token, TokenInfo, POS};

pub mod app;
pub mod application_state;
pub mod article;
pub mod compressor;
pub mod database;
pub mod file;
pub mod history;
pub mod reader;
pub mod state;
pub mod statistics;
pub mod token;
pub mod tokenizer;

fn load_application_state(path: &Path) -> Result<ApplicationState> {
    let contents = std::fs::read_to_string(path)?;

    Ok(ron::from_str(&contents)?)
}

fn save_application_state(path: &Path, application_state: ApplicationState) -> Result<()> {
    std::fs::write(path, ron::to_string(&application_state)?)?;

    Ok(())
}

pub fn main() -> Result<()> {
    let resources = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");

    let share_dir = resources.join("share");
    let config_dir = resources.join("config");
    let database_path = share_dir.join("reader.db");
    let bootstrap_path = share_dir.join("reader.ron");
    let files_dir = share_dir.join("imported");

    let database = Rc::new(RefCell::new(Database::new(&database_path)?));

    let initial_state = ApplicationState {
        reader_state: None,
        statistics_state: None,
        database,
        config_dir,
        share_dir,
        files_dir,
        current_view: View::Reader,
        unknown_tokens: Arc::new(Vec::new()),
    };

    launch_app(initial_state)?;

    Ok(())
}
