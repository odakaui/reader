use anyhow::Result;
use database::Database;
use std::path::Path;
use tokenizer::Tokenizer;

mod database;
mod tokenizer;

fn main() -> Result<()> {
    let path = Path::new("/tmp/reader.db");
    let db = Database::new(path)?;

    Ok(())
}
