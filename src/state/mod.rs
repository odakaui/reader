use druid::{Data, Lens};

pub use position::Position;
pub use operation::Operation;

pub mod position;
pub mod operation;

#[derive(Clone, Data, Debug, Lens)]
pub struct State {
    pub file_id: i32,
    pub position: Option<Position>,
    pub operation_num: i32,
    pub total: i32,
    pub unknown: i32,
    pub action: Operation,
}
