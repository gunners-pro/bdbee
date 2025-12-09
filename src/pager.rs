use crate::error::{DBError, Result};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
};

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

    pub fn read_page(&mut self, page_id: u64) -> Result<Vec<u8>> {
        let offset = page_id * self.page_size;
        let mut buffer = vec![0u8; self.page_size as usize];

        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(DBError::Io)?;

        self.file.read_exact(&mut buffer).map_err(DBError::Io)?;
        Ok(buffer)
    }

    pub fn write_page(&mut self, page_id: u64, data: &[u8]) -> Result<()> {
        if data.len() != self.page_size as usize {
            return Err(DBError::InvalidPageSize {
                expected: self.page_size,
                got: data.len(),
            });
        }

        let offset = page_id * self.page_size;
        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(DBError::Io)?;

        self.file.write_all(data).map_err(DBError::Io)?;

        Ok(())
    }
}
