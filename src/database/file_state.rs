use druid::{Data, Lens};
use std::sync::Arc;
use super::File;

#[derive(Clone, Debug, PartialEq, Data, Lens)]
pub struct FileState {
    pub files: Arc<Vec<File>>
}

impl FileState {
    pub fn new(files: &Vec<File>) -> Self {
        FileState {
            files: Arc::new(files.to_vec()),
        }
    }

    pub fn empty() -> Self {
        FileState {
            files: Arc::new(Vec::new()),
        }
    }
}
