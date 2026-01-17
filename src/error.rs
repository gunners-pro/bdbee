use std::fmt;

#[derive(Debug)]
pub enum DBError {
    Io(std::io::Error),
    InvalidPageSize { expected: u64, got: usize },
    InvalidMagic,
    InvalidVersion,
    InvalidTotalPages,
    InvalidPage,
    CorruptedFile,
    FileNotFound,
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DBError::Io(e) => write!(f, "I/O error: {}", e),
            DBError::InvalidPageSize { expected, got } => {
                write!(f, "Invalid page size (expected {}, got {})", expected, got)
            }
            DBError::InvalidMagic => write!(f, "Invalid database magic"),
            DBError::InvalidVersion => write!(f, "Unsupported database version"),
            DBError::InvalidTotalPages => write!(f, "Invalid total pages"),
            DBError::InvalidPage => write!(f, "Invalid page"),
            DBError::CorruptedFile => write!(f, "Corrupted database file"),
            DBError::FileNotFound => write!(f, "File not found"),
        }
    }
}

pub type Result<T> = std::result::Result<T, DBError>;
