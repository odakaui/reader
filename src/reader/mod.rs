use crate::{compressor, Article, Position, ReaderState, State};

// calculate the String to display in the left Label of the reader view
pub fn start(reader_state: &Option<ReaderState>) -> String {
    // return an empty string if current_state is none
    if reader_state.is_none() {
        return "".to_string();
    }

    let reader_state = reader_state.as_ref().unwrap();
    let current_state = &reader_state.current_state;

    // return an empty string if position is none (meaning eof)
    if current_state.position.is_none() {
        return "".to_string();
    }

    let position = current_state.position.as_ref().unwrap();

    if position.index == 0 {
        "".to_string()
    } else {
        let words = compressor::compress_line(&reader_state.article, &current_state);

        words[..position.index]
            .iter()
            .map(|x| x.text.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

// calculate the String to display in the center Label of the reader view
pub fn middle(reader_state: &Option<ReaderState>) -> String {
    // return an empty string if current_state is none
    if reader_state.is_none() {
        return "".to_string();
    }

    let reader_state = reader_state.as_ref().unwrap();
    let current_state = &reader_state.current_state;

    // return an empty string if position is none (meaning eof)
    if current_state.position.is_none() {
        return "EOF".to_string();
    }

    let word = compressor::compress(&reader_state.article, &current_state);

    word.text
}

// calculate the String to display in the right Label of the reader view
pub fn end(reader_state: &Option<ReaderState>) -> String {
    // return an empty string if current_state is none
    if reader_state.is_none() {
        return "".to_string();
    }

    let reader_state = reader_state.as_ref().unwrap();
    let current_state = &reader_state.current_state;

    // return an empty string if position is none (meaning eof)
    if current_state.position.is_none() {
        return "".to_string();
    }

    let position = current_state.position.as_ref().unwrap();

    let words = compressor::compress_line(&reader_state.article, &current_state);

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
pub fn next_position(article: &Article, state: &State) -> Option<Position> {
    let current_position = state.position.as_ref().expect("Failed to unwrap position");

    let article_length = article.lines.len();
    let line_length = compressor::compress_line(&article, state).len();

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
