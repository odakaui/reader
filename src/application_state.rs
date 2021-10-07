use crate::{Article, Database, History, State};
use druid::{Data, FontFamily, Lens};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Data, Lens)]
pub struct ApplicationState {
    pub font: Option<FontFamily>,
    pub current_state: Option<State>,

    #[data(ignore)]
    pub redo_stack: Vec<State>,

    #[data(ignore)]
    pub undo_stack: Vec<State>,

    #[data(ignore)]
    pub article: Article,

    #[data(ignore)]
    pub history: History,

    #[data(ignore)]
    pub database: Rc<RefCell<Database>>,
}
