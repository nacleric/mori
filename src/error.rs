use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error: failed to insert glyph {}", 0)]
    FailedToInsertGlyph(char),
}

impl From<char> for Error {
    fn from(c: char) -> Self {
        Self::FailedToInsertGlyph(c)
    }
}
