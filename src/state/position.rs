use druid::{Data, Lens};

#[derive(Clone, Data, Debug, Lens)]
pub struct Position {
    pub index: usize,
    pub line: usize,
}
