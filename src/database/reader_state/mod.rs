use super::{File, Position, State, Word};
use druid::{Data, Lens};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Data)]
pub enum Status {
    Empty,
    Eof,
    State
}

#[derive(Clone, Debug, Data, Lens)]
pub struct ReaderState {
    pub read: Arc<Vec<Word>>,
    pub unread: Arc<Vec<Word>>,
    pub current: Word,
    pub status: Status,
}

impl ReaderState {
    pub fn new(file: &File, state: &State) -> Self {
        if state.position.is_none() {
            return Self::eof()
        }

        let position = state.position.as_ref().unwrap();
        let words = words(file, position);

        let read = read(position, &words);
        let unread = unread(position, &words);
        let current = current(position, &words);

        println!("{:?}", current);

        ReaderState {
            read,
            unread,
            current,
            status: Status::State,
        }
    }

    pub fn empty() -> Self {
         ReaderState {
            read: Arc::new(Vec::new()),
            unread: Arc::new(Vec::new()),
            current: Word::empty(),
            status: Status::Empty,
        }
    }

    pub fn eof() -> Self {
         ReaderState {
            read: Arc::new(Vec::new()),
            unread: Arc::new(Vec::new()),
            current: Word::empty(),
            status: Status::Eof,
        }
    }
}

fn read(position: &Position, words: &[Word]) -> Arc<Vec<Word>> {
    let index = position.index;

    if words.is_empty() {
        return Arc::new(vec![Word::empty()])
    }

    Arc::new(words[..index].to_vec())
}

fn unread(position: &Position, words: &[Word]) -> Arc<Vec<Word>> {
    let index = position.index;

    if words.is_empty() {
        return Arc::new(vec![Word::empty()])
    }

    Arc::new(words[index + 1..].to_vec())
}

fn current(position: &Position, words: &[Word]) -> Word {
    let index = position.index;

    if words.is_empty() {
        return Word::empty()
    }

    words[index].clone()
}

fn words(file: &File, position: &Position) -> Vec<Word> {
    file.lines[position.line].words.to_vec()
}
