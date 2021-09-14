use anyhow::Result;
use database::Database;
use std::path::Path;

pub mod database;
pub mod data_types;
pub mod tokenizer;

fn main() -> Result<()> {
    let path = Path::new("/tmp/reader.db");
    let db = Database::new(path)?;

    Ok(())
}
