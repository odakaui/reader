use serde::{Deserialize, Serialize};

pub use line::Line;
pub use position::Position;

mod line;
mod position;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Article {
    pub name: String,
    pub lines: Vec<Line>,
}

impl Article {
    pub fn new(name: &str, lines: &Vec<Line>) -> Self {
        Article {
            name: name.into(),
            lines: lines.clone(),
        }
    }
}
