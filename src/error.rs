#[derive(Debug)]
pub enum DBError {
    Io(std::io::Error),
}

pub type Result<T> = std::result::Result<T, DBError>;
