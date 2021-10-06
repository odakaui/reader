use serde::{Deserialize, Serialize};

pub use line::Line;

mod line;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: i32,
    pub name: String,
    pub lines: Vec<Line>,
}

impl Article {
    pub fn new(id: i32, name: &str, lines: &Vec<Line>) -> Self {
        Article {
            id,
            name: name.into(),
            lines: lines.clone(),
        }
    }
}
