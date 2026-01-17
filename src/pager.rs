use crate::error::{DBError, Result};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

pub enum OpenMode {
    Create,
    Open,
}

pub struct Pager {
    file: File,
    page_size: u64,
    total_pages: u64,
}

struct Header {
    magic: [u8; 8],
    version: u64,
    page_size: u64,
    total_pages: u64,
}

impl Pager {
    pub fn open(path: &str, page_size: u64, mode: OpenMode) -> Result<Self> {
        let exists = Path::new(path).exists();

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .map_err(DBError::Io)?;

        let len = file.metadata().map_err(DBError::Io)?.len();
        let mut page_size_from_header = page_size;
        let mut total_pages = 1;

        match (exists, len, mode) {
            (false, 0, OpenMode::Create) => {
                Self::serialize_header(&file, page_size_from_header, total_pages)
            }
            (false, 0, OpenMode::Open) => {
                return Err(DBError::FileNotFound);
            }
            (true, 0, OpenMode::Open) => {
                return Err(DBError::CorruptedFile);
            }
            (_, len, _) if len >= page_size => {
                let mut buffer = vec![0u8; page_size_from_header as usize];
                file.seek(SeekFrom::Start(0)).unwrap();
                file.read_exact(&mut buffer).map_err(DBError::Io)?;

                let magic: [u8; 8] = buffer[0..8].try_into().unwrap();
                let version = u64::from_le_bytes(buffer[8..16].try_into().unwrap());
                page_size_from_header = u64::from_le_bytes(buffer[16..24].try_into().unwrap());
                total_pages = u64::from_le_bytes(buffer[24..32].try_into().unwrap());

                if magic != *b"BDBEE\0\0\0" {
                    return Err(DBError::InvalidMagic);
                }

                if version != 1 {
                    return Err(DBError::InvalidVersion);
                }

                if page_size_from_header != page_size {
                    return Err(DBError::InvalidPageSize {
                        expected: page_size,
                        got: page_size_from_header as usize,
                    });
                }

                if total_pages < 1 || total_pages * page_size > len {
                    return Err(DBError::InvalidTotalPages);
                }
            }
            _ => {
                return Err(DBError::CorruptedFile);
            }
        }

        Ok(Pager {
            file,
            page_size: page_size_from_header,
            total_pages,
        })
    }

    pub fn read_page(&mut self, page_id: u64) -> Result<Vec<u8>> {
        let offset = (page_id - 1) * self.page_size;
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

        let offset = (page_id - 1) * self.page_size;
        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(DBError::Io)?;

        self.file.write_all(data).map_err(DBError::Io)?;

        Ok(())
    }

    fn serialize_header(mut file: &File, page_size: u64, total_pages: u64) {
        let header = Header {
            magic: *b"BDBEE\0\0\0",
            version: 1,
            page_size,
            total_pages,
        };

        let mut buffer = vec![0u8; page_size as usize];
        buffer[0..8].copy_from_slice(&header.magic);
        buffer[8..16].copy_from_slice(&header.version.to_le_bytes());
        buffer[16..24].copy_from_slice(&header.page_size.to_le_bytes());
        buffer[24..32].copy_from_slice(&header.total_pages.to_le_bytes());

        file.write_all(&buffer).unwrap();
        file.flush().unwrap();
    }
}
