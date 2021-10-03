use serde::{Deserialize, Serialize};

pub use line::Line;
pub use position::Position;

mod line;
mod position;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Article {
    pub file_name: String,
    pub lines: Vec<Line>,
}
