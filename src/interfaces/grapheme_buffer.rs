use crate::{buffer::direction::Direction, position::Position, Result};

pub trait GraphemeBuffer {
    type Error: std::error::Error;

    fn content(&self) -> Vec<String>;
    fn delete_grapheme(&mut self, direction: Direction, pos: Position) -> (Position, Option<char>);
    fn insert_grapheme(&mut self, pos: Position, grapheme: char) -> Position;
    fn index(&self) -> usize;
    fn row_content(&self, pos: Position) -> &[u8];
    fn set_row_content(&mut self, pos: Position, data: String) -> Result<&mut Self, Self::Error>;
}