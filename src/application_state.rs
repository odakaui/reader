use crate::{Article, Database, History, State};
use druid::{Data, FontFamily, Lens};
use std::{cell::RefCell, path::PathBuf, rc::Rc};

#[derive(Clone, Data, Lens)]
pub struct ApplicationState {
    pub font: Option<FontFamily>,

    pub reader_state: Option<ReaderState>,

    #[data(ignore)]
    pub database: Rc<RefCell<Database>>,

    #[data(ignore)]
    pub config_dir: PathBuf,

    #[data(ignore)]
    pub share_dir: PathBuf,

    #[data(ignore)]
    pub files_dir: PathBuf,
}

#[derive(Clone, Data, Debug, Lens)]
pub struct ReaderState {
    pub current_state: State,

    #[data(ignore)]
    pub redo_stack: Vec<State>,

    #[data(ignore)]
    pub undo_stack: Vec<State>,

    #[data(ignore)]
    pub article: Article,

    #[data(ignore)]
    pub history: History,
}
