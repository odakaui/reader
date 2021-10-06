use druid::{Data, FontFamily, Lens};
use crate::{Article, State};

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
}

