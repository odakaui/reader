use super::{Connection, Deserialize, path, Serialize, history, anyhow, common, state, ReaderState, Token, Result};

pub use line::Line;

pub mod line;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    pub id: i32,
    pub name: String,
    pub lines: Vec<Line>
}

pub fn initialize(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )"#,
        [],
    )?;

    state::initialize(conn)?;
    history::initialize(conn)?;

    Ok(())
}

pub fn import(conn: &Connection, source_file: &path::PathBuf, target_dir: &path::PathBuf) -> Result<ReaderState>{
    let file_name = common::file_name(source_file)?;

    if exists(&file_name) {
        return Err(anyhow!("{} has already been imported"))
    }

    

    unimplemented!()
}

pub fn list() {
    unimplemented!()
}

pub fn load() {
    unimplemented!()
}

pub fn open() {
    unimplemented!()
}

pub fn delete() {
    unimplemented!()
}

fn exists(file_name: &str) -> bool {
    unimplemented!()
}
