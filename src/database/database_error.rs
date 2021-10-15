use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum DatabaseError {
    FileExists,
    Eof,
    UndoEmpty,
    RedoEmpty,
    FileOpen,
}

impl std::error::Error for DatabaseError {}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseError::FileExists => write!(f, "File Exists"),
            DatabaseError::Eof => write!(f, "End of File Reached"),
            DatabaseError::UndoEmpty => write!(f, "Undo Stack Empty"),
            DatabaseError::RedoEmpty => write!(f, "Undo Stack Empty"),
            DatabaseError::FileOpen => write!(f, "There is Currently No Open File"),
        }
    }
}
