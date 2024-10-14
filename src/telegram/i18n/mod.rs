pub mod inter;
pub mod language;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum I18nError {
    #[error("can't parse direntry")]
    IoError(#[from] std::io::Error),

    #[error("the data for key `{0}` is not available")]
    Redaction(String),

    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },

    #[error("unknown data store error")]
    Unknown,
}
