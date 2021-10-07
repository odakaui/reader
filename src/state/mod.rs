use druid::{Data, Lens};

pub use operation::Operation;
pub use position::Position;

pub mod operation;
pub mod position;

#[derive(Clone, Data, Debug, Lens)]
pub struct State {
    pub file_id: i32,
    pub position: Option<Position>,
    pub operation_num: i32,
    pub action: Option<Operation>,
}
