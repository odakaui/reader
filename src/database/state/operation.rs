#[derive(Clone, Debug)]
pub enum Operation {
    MarkKnown,
    MarkUnknown,
}

impl Operation {
    fn to_int(&self) -> i32 {
        match self {
            &Operation::MarkKnown => 0,
            &Operation::MarkUnknown => 1,
        }
    }

    fn from_int(i: i32) -> Operation {
        match i {
            0 => Operation::MarkKnown,
            _ => Operation::MarkUnknown,
        }
    }
}
