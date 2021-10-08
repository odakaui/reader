use anyhow::{anyhow, Result};
use app::launch_app;
use article::{Article, Line};
use database::Database;
use std::cell::RefCell;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::rc::Rc;
use token::{Token, POS};
use tokenizer::Tokenizer;

pub use application_state::{ApplicationState, ReaderState};
pub use history::History;
pub use state::{Operation, Position, State};

pub mod app;
pub mod application_state;
pub mod article;
pub mod compressor;
pub mod database;
pub mod file;
pub mod history;
pub mod reader;
pub mod state;
pub mod token;
pub mod tokenizer;

fn read_file(path: &Path) -> Result<String> {
    let f = fs::File::open(path)?;
    let mut buf = BufReader::new(f);
    let mut contents = String::new();
    buf.read_to_string(&mut contents)?;

    Ok(contents)
}

fn write_file(path: &Path, text: &str) -> Result<()> {
    fs::write(path, text)?;

    Ok(())
}

pub fn main() -> Result<()> {
    let resources = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");

    let share_dir = resources.join("share");
    let config_dir = resources.join("config");
    let database_path = share_dir.join("reader.db");
    let files_dir = share_dir.join("imported");

    let database = Rc::new(RefCell::new(Database::new(&database_path)?));

    let initial_state = ApplicationState {
        font: None,
        reader_state: None,
        database,
        config_dir,
        share_dir,
        files_dir,
    };

    launch_app(initial_state)?;

    Ok(())
}
