use super::{State, Line, File, Token};

pub use word::Word;

mod word;

pub struct ReaderState {
    pub read: Vec<Word>,
    pub unread: Vec<Word>,
    pub current: Word,
}

impl ReaderState {
    pub fn new(file: &File, state: &State) -> Self {
        let line = line(file, state); 
        let words = word::to_words(&line);

        let read = read(state, &words);
        let unread = unread(state, &words);
        let current = current(state, &words);

        ReaderState {
            read,
            unread,
            current,
        }
    }
}

fn read(state: &State, words: &Vec<Word>) -> Vec<Word> {
    let index = state.current_index;

    words[..index].to_vec()
}

fn unread(state: &State, words: &Vec<Word>) -> Vec<Word> {
    let index = state.current_index;

    words[index + 1..].to_vec()
}

fn current(state: &State, words: &Vec<Word>) -> Word {
    let index = state.current_index;

    words[index]
}

fn line(file: &File, state: &State) -> Line {
    file.lines[state.current_line]
}
