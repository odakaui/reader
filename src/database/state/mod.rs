use super::{Connection, Result};

pub use operation::Operation;

pub mod operation;

#[derive(Clone, Debug)]
pub struct State {
    pub id: i32,
    pub history_id: i32,
    pub current_index: usize,
    pub current_line: usize,
    pub operation_num: i32,
    pub action: Option<Operation>,
}

pub fn initialize(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS state (
            id INTEGER PRIMARY KEY,
            history_id INTEGER NOT NULL,
            current_index INTEGER,
            current_line INTEGER,
            operation_num INTEGER NOT NULL,
            action INTEGER,
            CONSTRAINT uq_state UNIQUE(file_id, operation_num)
            FOREIGN KEY (file_id) REFERENCES files(id)
        )"#,
        []
    )?;

    Ok(())
}
