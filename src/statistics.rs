use crate::{ApplicationState, StatisticsState};
use anyhow::Result;
use std::sync::Arc;

pub fn statistics(data: &mut ApplicationState) -> Result<()> {
    // return if reader_state is None
    if data.reader_state.is_none() {
        return Ok(());
    }

    let reader_state = data.reader_state.as_ref().unwrap();
    let file_name = &reader_state.article.name;

    let start_date = reader_state.history.start_date;
    let end_date = reader_state.history.end_date;

    let history_id = reader_state.history.id;

    let mut database = data.database.borrow_mut();
    let total_seen = database.select_total_seen_for_history_id(history_id)?;
    let total_unknown = database.select_total_unknown_for_history_id(history_id)?;

    let unknown_tokens = database.select_unknown_for_history(history_id)?;

    let statistics_state = StatisticsState {
        file_name: file_name.to_string(),

        start_date,
        end_date,

        total_seen,
        total_unknown,
    };

    data.statistics_state = Some(statistics_state);
    data.unknown_tokens = Arc::new(unknown_tokens);

    Ok(())
}
