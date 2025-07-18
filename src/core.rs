#![allow(dead_code)]

/// Custom `Result` type returned by TurboCache and its op's
pub type TurboResult<T> = Result<T, TurboError>;

#[derive(Debug)]
pub enum TurboError {
    /// An I/O error occurred.
    Io(std::io::Error),

    /// Key size out of range
    KeyTooLarge(usize),

    /// Value size out of range
    ValueTooLarge(usize),

    /// Invalid buffer or shard file
    InvalidFile,
}

impl From<std::io::Error> for TurboError {
    fn from(err: std::io::Error) -> Self {
        TurboError::Io(err)
    }
}

impl std::error::Error for TurboError {}

impl std::fmt::Display for TurboError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TurboError::Io(err) => write!(f, "I/O error: {}", err),
            TurboError::KeyTooLarge(size) => write!(f, "Key size ({}) is too large", size),
            TurboError::ValueTooLarge(size) => write!(f, "Value size ({}) is too large", size),
            TurboError::InvalidFile => write!(f, "Invalid shard/buffer file"),
        }
    }
}
