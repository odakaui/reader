use druid::{Data, Lens};
use std::{cell::RefCell, rc::Rc};

use super::{Database, ReaderState, StatisticsState};

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
    pub statistics_state: StatisticsState,
    pub current_view: View,

    #[data(ignore)]
    pub database: Rc<RefCell<Database>>,
}
