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

impl Article {
    // calculate the String to display in the left Label of the reader view
    pub fn calculate_start(&self, position: &Position) -> String {
        if position.index == 0 {
            "".to_string()
        } else {
            let tokens = self.lines[position.line].tokens[..position.index].to_vec();
            tokens
                .iter()
                .map(|x| x.text.to_string())
                .collect::<Vec<String>>()
                .join("")
        }
    }

    // calculate the String to display in the center Label of the reader view
    pub fn calculate_middle(&self, position: &Position) -> String {
        self.lines[position.line].tokens[position.index]
            .text
            .to_string()
    }

    // calculate the String to display in the right Label of the reader view
    pub fn calculate_end(&self, position: &Position) -> String {
        let tokens = self.lines[position.line].tokens.clone();

        if position.index >= tokens.len() {
            "".to_string()
        } else {
            let slice = tokens[position.index + 1..].to_vec();
            slice
                .iter()
                .map(|x| x.text.to_string())
                .collect::<Vec<String>>()
                .join("")
        }
    }

    // calculate the next available index
    // returns None if the next position is past the end of the file
    pub fn next_position(&self, current_position: &Position) -> Option<Position> {
        let article_length = self.lines.len();
        let line_length = self.lines[current_position.line].tokens.len();

        let mut is_eof = false;

        let new_index: usize;
        let new_line: usize;

        if current_position.index + 1 >= line_length {
            new_index = 0;

            if current_position.line + 1 >= article_length {
                new_line = 0;
                is_eof = true;
            } else {
                new_line = current_position.line + 1;
            }
        } else {
            new_index = current_position.index + 1;
            new_line = current_position.line;
        }

        if is_eof {
            None
        } else {
            Some(Position {
                index: new_index,
                line: new_line,
            })
        }
    }
}
