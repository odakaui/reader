use crate::{Article, Database, History, State, TokenInfo};
use chrono::{DateTime, Utc};
use druid::{Data, Lens};
use std::{cell::RefCell, path::PathBuf, rc::Rc, sync::Arc};

#[derive(Clone, Debug, Data, PartialEq)]
pub enum View {
    Empty,
    Reader,
    Statistics,
}

#[derive(Clone, Data, Lens)]
pub struct ApplicationState {
    pub reader_state: Option<ReaderState>,
    pub statistics_state: Option<StatisticsState>,

    #[data(ignore)]
    pub database: Rc<RefCell<Database>>,

    #[data(ignore)]
    pub config_dir: PathBuf,

    #[data(ignore)]
    pub share_dir: PathBuf,

    #[data(ignore)]
    pub files_dir: PathBuf,

    pub current_view: View,

    pub unknown_tokens: Arc<Vec<TokenInfo>>,
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

#[derive(Clone, Data, Debug, Lens)]
pub struct StatisticsState {
    pub file_name: String,

    #[data(ignore)]
    pub start_date: DateTime<Utc>,

    #[data(ignore)]
    pub end_date: Option<DateTime<Utc>>,

    pub total_seen: i32,
    pub total_unknown: i32,
}
