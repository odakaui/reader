use anyhow::Result;
use druid::Data;
use rusqlite::{params, Connection};

const ID: i32 = 1;

#[derive(Clone, Debug, PartialEq, Data)]
pub enum Filter {
    All,
    Learned,
    Unlearned,
}

impl Filter {
    pub fn to_int(&self) -> i32 {
        match self {
            Filter::All => 0,
            Filter::Learned => 1,
            Filter::Unlearned => 2,
        }
    }

    pub fn from_int(i: i32) -> Self {
        match i {
            1 => Filter::Learned,
            2 => Filter::Unlearned,
            _ => Filter::All,
        }
    }
}

pub fn initialize(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS current_filter (
            id INTEGER PRIMARY KEY,
            filter INTEGER NOT NULL
        )"#,
        [],
    )?;

    conn.execute(
        r#"INSERT OR IGNORE INTO current_filter (id, filter) VALUES (?1, ?2)"#,
        params![ID, Filter::All.to_int()],
    )?;

    Ok(())
}

pub fn set_filter(conn: &Connection, filter: &Filter) -> Result<()> {
    conn.execute(
        r#"UPDATE current_filter SET filter=?1 WHERE id=?2"#,
        params![filter.to_int(), ID],
    )?;

    Ok(())
}

pub fn get_filter(conn: &Connection) -> Result<Filter> {
    Ok(Filter::from_int(conn.query_row(
        r#"SELECT filter FROM current_filter WHERE id=?1"#,
        params![ID],
        |row| row.get::<usize, i32>(0),
    )?))
}
