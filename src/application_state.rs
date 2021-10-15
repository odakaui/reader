use druid::{Data, Lens};
use std::{cell::RefCell, rc::Rc};

use super::{Database, ReaderState};

#[derive(Clone, Debug, Data, PartialEq)]
pub enum View {
    Empty,
    Eof,
    Reader,
    Statistics,
}

#[derive(Clone, Data, Lens)]
pub struct ApplicationState {
    pub reader_state: ReaderState,
    pub current_view: View,

    #[data(ignore)]
    pub database: Rc<RefCell<Database>>,
}
