use super::{history_token, token};
use super::{HistoryToken, Status, Token};
use chrono::{DateTime, Utc};
use druid::{Data, Lens};
use std::sync::Arc;

pub use token_info::TokenInfo;

mod token_info;

#[derive(Clone, Debug, PartialEq, Data, Lens)]
pub struct StatisticsState {
    pub name: String,

    #[data(ignore)]
    pub start_date: DateTime<Utc>,
    #[data(ignore)]
    pub end_date: Option<DateTime<Utc>>,

    pub total_seen: i32,
    pub total_unknown: i32,
    pub unknown: Arc<Vec<TokenInfo>>,
}

impl StatisticsState {
    pub fn empty() -> Self {
        StatisticsState { name: String::new(), start_date: Utc::now(), end_date: None, total_seen: 0, total_unknown: 0, unknown: Arc::new(Vec::new()) }
    }
    pub fn percent_known(&self) -> i32 {
        let total_seen = self.total_seen as f64;
        let total_known = (self.total_seen - self.total_unknown) as f64;

        (total_known / total_seen * 100.).round() as i32
    }
}
