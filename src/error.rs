#[derive(Debug)]
pub enum DBError {
    Io(std::io::Error),
    InvalidPageSize { expected: u64, got: usize },
}

pub type Result<T> = std::result::Result<T, DBError>;
