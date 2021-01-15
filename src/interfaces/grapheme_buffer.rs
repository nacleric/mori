use crate::{buffer::direction::Direction, position::Position, Result};
use crate::interfaces::Bufferable;
pub trait GraphemeBuffer: Bufferable {
    type Error: std::error::Error;

    fn content(&self) -> Vec<String>;
    fn delete_grapheme(&mut self, direction: Direction, pos: Position) -> (Position, Option<char>);
    fn delete_graphemes(&mut self) -> (Position, Vec<String>);
    fn insert_grapheme(&mut self, pos: Position, grapheme: char) -> Position;
    fn insert_graphemes<I: Iterator<Item = char>>(
        &mut self,
        pos: Position,
        graphemes: I,
    ) -> Position;
    fn index(&self, pos: Position) -> usize;
    fn row_content(&self, pos: Position) -> &[u8];
    fn set_row_content(&mut self, pos: Position, data: String) -> Result<&mut Self, Self::Error>;
}
