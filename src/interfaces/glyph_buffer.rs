use crate::{
    position::Position,
    Result,
    buffer::direction::Direction,
};

pub trait GlyphBuffer {
    type Error: std::error::Error;

    fn content(&self) -> Vec<String>;
    fn delete_glyph(&mut self, direction: Direction, pos: Position) -> (Position, Option<char>);
    fn insert_glyph(&mut self, pos: Position, glyph: char) -> Position;
    fn index(&self) -> usize;
    fn row_content(&self, pos: Position) -> &[u8];
    fn set_row_content(&mut self, pos: Position, data: String) -> Result<&mut Self, Self::Error>;
}
