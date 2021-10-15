use super::{common, history, history_token, state};
use super::{DatabaseError, Operation, Position, State, Token, Tokenizer, POS};
use anyhow::{anyhow, Result, Context};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::{fs, path};

pub use line::Line;
pub use word::Word;

pub mod current_file;
pub mod line;
pub mod word;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    pub id: i32,
    pub name: String,
    pub lines: Vec<Line>,
}

pub fn initialize(conn: &Connection, target_dir: &path::PathBuf) -> Result<()> {
    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )"#,
        [],
    )?;

    common::create_dir(target_dir)?;

    current_file::initialize(conn)?;
    history::initialize(conn)?;

    Ok(())
}

pub fn insert_file(
    conn: &Connection,
    source_file: &path::PathBuf,
    target_dir: &path::PathBuf,
) -> Result<File> {
    let name = common::file_name(source_file)?;

    if exists(conn, &target_dir, &name) {
        return Err(DatabaseError::FileExists.into());
    }

    save_file(source_file, target_dir, &name)?;

    conn.execute(
        r#"INSERT OR IGNORE INTO files (name)
            VALUES (?1)"#,
        params![name],
    )?;

    let id = conn.last_insert_rowid() as i32;
    let file = select_file(conn, target_dir, id)?;

    history::insert_history(conn, file.id)?;

    Ok(file)
}

pub fn select_file(conn: &Connection, target_dir: &path::PathBuf, id: i32) -> Result<File> {
    let file = conn.query_row(
        r#"SELECT id, name FROM files
                WHERE id=?1"#,
        params![id],
        |row| {
            let name: String = row.get(1)?;
            let lines = load_file(target_dir, &name)
                .expect(&format!("{} doesn't exist. Database is broken.", name));

            Ok(File {
                id: row.get(0)?,
                name,
                lines,
            })
        },
    )?;

    current_file::set_current_file(conn, file.id)?;

    Ok(file)
}

pub fn reset(conn: &Connection, file: &File) -> Result<State> {
    let history_id = history::current_history(conn, file.id)?;
    history::reset_history(conn, file.id, history_id)?;

    Ok(current_state(conn, file)?)
}

pub fn next(conn: &Connection, file: &File, action: &Operation) -> Result<State> {
    let mut current_state = current_state(conn, file)?;
    current_state.action = Some(action.to_owned());

    if current_state.position.is_none() {
        return Err(DatabaseError::Eof.into());
    }

    let line = current_state.position.as_ref().unwrap().line;
    let index = current_state.position.as_ref().unwrap().index;

    let lines = &file.lines;
    let words = &lines[line].words;

    let new_index: usize;
    let new_line: usize;
    let mut eof = false;

    if index + 1 >= words.len() {
        new_index = 0;

        if line + 1 >= lines.len() {
            new_line = 0;
            eof = true;
        } else {
            new_line = line + 1;
        }
    } else {
        new_index = index + 1;
        new_line = line;
    }

    let position = match eof {
        true => None,
        false => Some(Position {
            index: new_index,
            line: new_line,
        }),
    };

    let next_state = state::next_state(conn, &current_state, &position)?;

    let word = word(file, &current_state)?;
    let is_unknown = action == &Operation::MarkUnknown;
    history_token::insert_history_tokens(conn, current_state.history_id, &word.tokens, is_unknown)?;

    Ok(next_state)
}

pub fn undo(conn: &Connection, file: &File) -> Result<State> {
    let current_state = current_state(conn, file)?;

    let previous_state = state::undo_state(conn, &current_state).context("failed to select previous state.")?;

    let word = word(file, &previous_state).context("failed to retreieve word.")?;
    let is_unknown = previous_state
        .action
        .as_ref()
        .expect("previous_state has no action")
        == &Operation::MarkUnknown;
    history_token::delete_history_tokens(
        conn,
        previous_state.history_id,
        &word.tokens,
        is_unknown,
    ).context("failed to delete tokens.")?;

    Ok(previous_state)
}

pub fn redo(conn: &Connection, file: &File) -> Result<State> {
    let current_state = current_state(conn, file)?;
    let action = current_state
        .action
        .as_ref()
        .ok_or(anyhow!("current_state has no action"))?;

    let word = word(file, &current_state)?;
    let is_unknown = action == &Operation::MarkUnknown;
    history_token::insert_history_tokens(conn, current_state.history_id, &word.tokens, is_unknown)?;

    Ok(state::redo_state(conn, &current_state)?)
}

pub fn current_file(conn: &Connection, target_dir: &path::PathBuf) -> Result<File> {
    let id = current_file::get_current_file(conn)?.ok_or(DatabaseError::FileOpen)?;

    Ok(select_file(conn, target_dir, id)?)
}

pub fn current_state(conn: &Connection, file: &File) -> Result<State> {
    let history_id = history::current_history(conn, file.id)?;
    let state_id = state::current_state(conn, history_id)?;

    let state = state::select_state(conn, state_id)?;

    Ok(state)
}

fn save_file(source_file: &path::PathBuf, target_dir: &path::PathBuf, name: &str) -> Result<()> {
    let mut tokenizer = Tokenizer::new()?;
    let lines: Vec<Line> = common::clean_text(&fs::read_to_string(source_file)?)
        .into_iter()
        .map(|sentence| -> Result<Line> { Ok(Line::new(&mut tokenizer, &sentence)?) })
        .flatten()
        .collect();

    let target_file = target_dir.join(name);
    fs::write(target_file, ron::to_string(&lines)?)?;

    Ok(())
}

fn load_file(target_dir: &path::PathBuf, name: &str) -> Result<Vec<Line>> {
    let target_file = target_dir.join(name);

    Ok(ron::from_str(&fs::read_to_string(target_file)?)?)
}

fn exists(conn: &Connection, target_dir: &path::PathBuf, file_name: &str) -> bool {
    let target_file = target_dir.join(file_name);

    if target_file.exists() {
        return true;
    }

    conn.query_row(
        r#"SELECT id FROM files WHERE name=?1"#,
        params![file_name],
        |row| Ok(row.get::<usize, i32>(0)),
    )
    .is_ok()
}

fn word(file: &File, state: &State) -> Result<Word> {
    let position = state.position.as_ref().ok_or(DatabaseError::Eof)?;
    let index = position.index;
    let line = position.line;

    if file.lines[line].words.is_empty() {
        Ok(Word::empty())
    } else {
        Ok(file.lines[line].words[index].clone())
    }
}
