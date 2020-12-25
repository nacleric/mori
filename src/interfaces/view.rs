use crate::Result;
use std::io::Write;

pub trait View {
    fn show<W: Write>(&self, writer: &mut W) -> Result<&Self>
    where
        Self: Sized;
}
