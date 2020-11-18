use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum Error {
    #[error("Error: failed to insert glyph {}", 0)]
    FailedToInsertGlyph(char),
    #[error("Error: Invalid `Position` specified {}", 0)]
    InvalidPosition(crate::Position),
    #[error("IoError: {}", 0)]
    Foo(String),
}

impl From<char> for Error {
    fn from(c: char) -> Self {
        Self::FailedToInsertGlyph(c)
    }
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Self::Foo(io_error.to_string())
    }
}
