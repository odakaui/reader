use druid::Data;

#[derive(Clone, Data, Debug, PartialEq)]
pub enum Operation {
    MarkKnown,
    MarkUnknown,
}

impl Operation {
    pub fn from_int(i: i32) -> Self {
        match i {
            0 => Operation::MarkKnown,
            _ => Operation::MarkUnknown
        }
    }

    pub fn number(&self) -> i32 {
        match self {
            Operation::MarkKnown => 0,
            Operation::MarkUnknown => 1,
        }
    }
}
