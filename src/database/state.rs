use anyhow::Result;
use rusqlite::{params, Connection};

#[derive(Debug)]
enum Operation {
    MarkKnown,
    MarkUnknown,
}

impl Operation {
    fn to_int(&self) -> i32 {
        match self {
            &Operation::MarkKnown => 0,
            &Operation::MarkUnknown => 1,
        }
    }

    fn from_int(i: i32) -> Operation {
        match i {
            0 => Operation::MarkKnown,
            _ => Operation::MarkUnknown,
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub id: i32,
    pub history_id: i32,
    pub current_index: Option<usize>,
    pub current_line: Option<usize>,
    pub operation_num: i32,
    pub action: Option<Operation>,
}

impl State {
    pub fn new(
        id: i32,
        history_id: i32,
        current_index: Option<usize>,
        current_line: Option<usize>,
        operation_num: i32,
        action: Option<Operation>,
    ) -> Self {
        State {
            id,
            history_id,
            current_index,
            current_line,
            operation_num,
            action,
        }
    }
}

pub fn insert_state(conn: &Connection, state: &State) -> Result<()> {
    let action = state.action.as_ref().map(|action| action.to_int());

    conn.execute(
        r#"INSERT OR IGNORE INTO state (history_id, current_index, current_line, operation_num, action)
            VALUES (?1, ?2, ?3, ?4, ?5)"#,
        params![state.history_id, state.current_index, state.current_line, state.operation_num, action])?;

    Ok(())
}

pub fn insert_initial_state(conn: &Connection, history_id: i32) -> Result<()> {
    let state = State {
        id: 0,
        history_id,
        current_index: Some(0),
        current_line: Some(0),
        operation_num: 1,
        action: None,
    };

    insert_state(conn, &state)?;

    Ok(())
}

pub fn delete_state(conn: &Connection, history_id: i32) -> Result<()> {
    conn.execute(
        r#"DELETE FROM state WHERE history_id=?1"#,
        params![history_id],
    )?;

    Ok(())
}

pub fn delete_state_operation_num(
    conn: &Connection,
    history_id: i32,
    operation_num: i32,
) -> Result<()> {
    conn.execute(
        r#"DELETE FROM state WHERE history_id=?1 AND operation_num>?2"#,
        params![history_id, operation_num],
    )?;

    Ok(())
}

pub fn select_state(conn: &Connection, history_id: i32) -> Result<Vec<State>> {
    let mut stmt = conn.prepare(
        r#"SELECT id, history_id, current_index, current_line, operation_num, action FROM state
                                    WHERE history_id=?1"#,
    )?;

    Ok(stmt
        .query_map(params![history_id], |row| {
            let action = row
                .get::<usize, Option<i32>>(5)?
                .as_ref()
                .map(|action| Operation::from_int(*action));

            Ok(State {
                id: row.get(0)?,
                history_id: row.get(1)?,
                current_index: row.get(2)?,
                current_line: row.get(3)?,
                operation_num: row.get(4)?,
                action,
            })
        })?
        .flatten()
        .collect())
}

pub fn select_state_for_operation_num(
    conn: &Connection,
    history_id: i32,
    operation_num: i32,
) -> Result<State> {
    Ok(conn.query_row(
        r#"SELECT history_id, current_index, current_line, operation_num, action
            FROM state WHERE history_id=?1 AND operation_num=?2"#,
        params![history_id, operation_num],
        |row| {
            let action = row
                .get::<usize, Option<i32>>(5)?
                .as_ref()
                .map(|action| Operation::from_int(*action));

            Ok(State {
                id: row.get(0)?,
                history_id: row.get(1)?,
                current_index: row.get(2)?,
                current_line: row.get(3)?,
                operation_num: row.get(4)?,
                action,
            })
        },
    )?)
}

pub fn select_state_current(conn: &Connection, history_id: i32, operation_num: i32) -> Result<State> {
    Ok(conn.query_row(
        r#"SELECT history_id, current_index, current_line, operation_num, action
            FROM state WHERE history_id=?1 AND operation_num=?2"#,
        params![history_id, operation_num],
        |row| {
            let action = row
                .get::<usize, Option<i32>>(5)?
                .as_ref()
                .map(|action| Operation::from_int(*action));

            Ok(State {
                id: row.get(0)?,
                history_id: row.get(1)?,
                current_index: row.get(2)?,
                current_line: row.get(3)?,
                operation_num: row.get(4)?,
                action,
            })
        },
    )?)
}
