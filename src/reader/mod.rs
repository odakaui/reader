use crate::{compressor, Article, Position};

// calculate the String to display in the left Label of the reader view
pub fn calculate_start(article: &Article, position: &Position) -> String {
    if position.index == 0 {
        "".to_string()
    } else {
        let words = compressor::compress_line(&article.lines[position.line]);

        words[..position.index]
            .iter()
            .map(|x| x.text.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

// calculate the String to display in the center Label of the reader view
pub fn calculate_middle(article: &Article, position: &Position) -> String {
    let words = compressor::compress_line(&article.lines[position.line]);

    words[position.index].text.to_string()
}

// calculate the String to display in the right Label of the reader view
pub fn calculate_end(article: &Article, position: &Position) -> String {
    let words = compressor::compress_line(&article.lines[position.line]);

    if position.index >= words.len() {
        "".to_string()
    } else {
        words[position.index + 1..]
            .iter()
            .map(|x| x.text.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

// calculate the next available index
// returns None if the next position is past the end of the file
pub fn next_position(article: &Article, current_position: &Position) -> Option<Position> {
    let article_length = article.lines.len();
    let line_length = compressor::compress_line(&article.lines[current_position.line]).len();

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
