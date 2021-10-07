use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq)]
pub struct History {
    pub id: i32,
    pub file_id: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}
