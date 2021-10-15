use super::DatabaseError;
use anyhow::{anyhow, Result};
use rusqlite::{params, Connection};

pub use operation::Operation;

pub mod current_state;
pub mod operation;

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub index: usize,
    pub line: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub id: i32,
    pub history_id: i32,
    pub position: Option<Position>,
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
            CONSTRAINT uq_state UNIQUE(history_id, operation_num)
            FOREIGN KEY (history_id) REFERENCES history (id)
        )"#,
        [],
    )?;

    current_state::initialize(conn)?;

    Ok(())
}

pub fn initial_state(conn: &Connection, history_id: i32) -> Result<State> {
    let initial_position = Position { line: 0, index: 0 };
    let initial_state = State {
        id: 0,
        history_id,
        position: Some(initial_position),
        operation_num: 0,
        action: None,
    };

    let state = insert_state(conn, &initial_state)?;

    Ok(state)
}

pub fn next_state(
    conn: &Connection,
    current_state: &State,
    new_position: &Option<Position>,
) -> Result<State> {
    let next_state = State {
        id: current_state.id,
        history_id: current_state.history_id,
        position: new_position.clone(),
        operation_num: current_state.operation_num + 1,
        action: None,
    };

    clear_redo_stack(conn, current_state.history_id)?;

    update_state(conn, current_state)?;
    insert_state(conn, &next_state)
}

pub fn current_state(conn: &Connection, history_id: i32) -> Result<i32> {
    Ok(current_state::get_current_state(conn, history_id)?)
}

pub fn reset_state(conn: &Connection, history_id: i32) -> Result<()> {
    conn.execute(
        r#"DELETE FROM state WHERE history_id=?1"#,
        params![history_id],
    )?;

    current_state::delete_current_state(conn, history_id)?;

    Ok(())
}

pub fn undo_state(conn: &Connection, state: &State) -> Result<State> {
    let history_id = state.history_id;
    let operation_num = state.operation_num - 1;

    let previous_state = conn.query_row(
        r#"SELECT id, history_id, current_index, current_line, operation_num, action FROM state WHERE history_id=?1 AND operation_num=?2"#,
        params![history_id, operation_num],
        |row| {
            let index = row.get::<usize, Option<usize>>(2)?;
            let line = row.get::<usize, Option<usize>>(3)?;

            let position = if index.is_none() || line.is_none() {
                None
            } else {
                Some(Position {
                    index: index.unwrap(),
                    line: line.unwrap(),
                })
            };

            Ok(State {
                id: row.get(0)?,
                history_id: row.get(1)?,
                position,
                operation_num: row.get(4)?,
                action: row.get::<usize, Option<i32>>(5)?.map(|action| Operation::from_int(action)),
            })
        }).or(Err(DatabaseError::UndoEmpty))?;

    current_state::set_current_state(conn, history_id, previous_state.id)?;

    Ok(previous_state)
}

pub fn redo_state(conn: &Connection, state: &State) -> Result<State> {
    let history_id = state.history_id;
    let operation_num = state.operation_num + 1;

    let next_state = conn.query_row(
        r#"SELECT id, history_id, current_index, current_line, operation_num, action FROM state WHERE history_id=?1 AND operation_num=?2"#,
        params![history_id, operation_num],
        |row| {
            let index = row.get::<usize, Option<usize>>(2)?;
            let line = row.get::<usize, Option<usize>>(3)?;

            let position = if index.is_none() || line.is_none() {
                None
            } else {
                Some(Position {
                    index: index.unwrap(),
                    line: line.unwrap(),
                })
            };

            Ok(State {
                id: row.get(0)?,
                history_id: row.get(1)?,
                position,
                operation_num: row.get(4)?,
                action: row.get::<usize, Option<i32>>(5)?.map(|action| Operation::from_int(action)),
            })
        }).or(Err(DatabaseError::RedoEmpty))?;

    current_state::set_current_state(conn, history_id, next_state.id)?;

    Ok(next_state)
}

pub fn select_state(conn: &Connection, id: i32) -> Result<State> {
    Ok(conn.query_row(
        r#"SELECT id, history_id, current_index, current_line, operation_num, action FROM state WHERE id=?1"#,
        params![id],
        |row| {
            let index = row.get::<usize, Option<usize>>(2)?;
            let line = row.get::<usize, Option<usize>>(3)?;

            let position = if index.is_none() || line.is_none() {
                None
            } else {
                Some(Position {
                    index: index.unwrap(),
                    line: line.unwrap(),
                })
            };

            Ok(State {
                id: row.get(0)?,
                history_id: row.get(1)?,
                position,
                operation_num: row.get(4)?,
                action: row.get::<usize, Option<i32>>(5)?.map(|action| Operation::from_int(action)),
            })
        })?
    )
}

fn insert_state(conn: &Connection, state: &State) -> Result<State> {
    let index = match &state.position {
        None => None,
        Some(position) => Some(position.index),
    };

    let line = match &state.position {
        None => None,
        Some(position) => Some(position.line),
    };

    conn.execute(
        r#"INSERT OR IGNORE INTO state (history_id, current_index, current_line, operation_num, action)
            VALUES (?1, ?2, ?3, ?4, ?5)"#,
        params![state.history_id, index, line, state.operation_num, state.action.as_ref().map(|action| action.to_int())]
    )?;

    let id = conn.last_insert_rowid() as i32;
    let state = select_state(conn, id)?;

    current_state::set_current_state(conn, state.history_id, id)?;

    Ok(state)
}

fn clear_redo_stack(conn: &Connection, history_id: i32) -> Result<()> {
    let state = select_state(conn, current_state(conn, history_id)?)?;
    let operation_num = state.operation_num;
    let history_id = state.history_id;

    conn.execute(
        r#"DELETE FROM state WHERE history_id=?1 AND operation_num>?2"#,
        params![history_id, operation_num],
    )?;

    Ok(())
}

fn update_state(conn: &Connection, state: &State) -> Result<()> {
    conn.execute(
        r#"UPDATE state SET action=?1 WHERE id=?2"#,
        params![
            state.action.as_ref().expect("action is not set").to_int(),
            state.id
        ],
    )?;

    Ok(())
}
