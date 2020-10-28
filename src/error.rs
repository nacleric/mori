use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum Error {
    #[error("Error: failed to insert glyph {}", 0)]
    FailedToInsertGlyph(char),
    #[error("Error: Invalid `Position` specified {}", 0)]
    InvalidPosition(crate::Position),
}

impl From<char> for Error {
    fn from(c: char) -> Self {
        Self::FailedToInsertGlyph(c)
    }
}
