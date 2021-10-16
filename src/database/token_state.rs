use super::TokenInfo;
use druid::{Data, Lens};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Data)]
pub enum Sort {
    Unknown,
    Percent,
    Total,
}

#[derive(Clone, Debug, PartialEq, Data)]
pub enum Filter {
    All,
    Learned,
    Unlearned,
}

#[derive(Clone, Debug, Data, Lens)]
pub struct TokenState {
    pub tokens: Arc<Vec<TokenInfo>>,
    pub sort: Sort,
    pub filter: Filter,
    pub reverse: bool,
}

impl TokenState {
    pub fn new(tokens: &Vec<TokenInfo>, sort: &Sort, filter: &Filter) -> Self {
        TokenState {
            tokens: Arc::new(tokens.clone()),
            sort: sort.clone(),
            filter: filter.clone(),
            reverse: true,
        }
    }

    pub fn empty() -> Self {
        let info: Vec<TokenInfo> = Vec::new();
        TokenState {
            tokens: Arc::new(info),
            sort: Sort::Total,
            filter: Filter::All,
            reverse: true,
        }
    }
}
