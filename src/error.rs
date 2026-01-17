#[derive(Debug)]
pub enum DBError {
    Io(std::io::Error),
    InvalidPageSize { expected: u64, got: usize },
    InvalidMagic,
    InvalidVersion,
    InvalidTotalPages,
    CorruptedFile,
    FileNotFound,
}

pub type Result<T> = std::result::Result<T, DBError>;
