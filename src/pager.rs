use crate::error::{DBError, Result};
use std::fs::{File, OpenOptions};

pub struct Pager {
    file: File,
    page_size: u64,
}

impl Pager {
    pub fn open(path: &str, page_size: u64) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .map_err(DBError::Io)?;

        Ok(Pager { file, page_size })
    }
}
