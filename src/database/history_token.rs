pub struct HistoryToken {
    pub history_id: i32,
    pub token_id: i32,
    pub total_unknown: i32,
    pub total_seen: i32,
}

impl HistoryToken {
    pub fn new(history_id: i32, token_id: i32, total_unknown: i32, total_seen: i32) -> Self {
        HistoryToken {
            history_id,
            token_id,
            total_unknown,
            total_seen,
        }
    }
}
