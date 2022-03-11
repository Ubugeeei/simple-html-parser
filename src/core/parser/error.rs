use combine::error::StringStreamError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum HTMLParseError {
    #[error("failed to parse; {0}")]
    InvalidResourceError(StringStreamError),
}
