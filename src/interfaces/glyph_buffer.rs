use crate::{
    position::Position,
    Result,
    buffer::direction::Direction,
};

pub trait GlyphBuffer {
    type Error: std::error::Error;

    fn content(&self, pos: Position) -> Vec<String>;
    fn delete_glyph(&mut self, direction: Direction, pos: Position) -> Option<char>;
    fn insert_glyph(&mut self, glyph: char, pos: Position) -> Position;
    fn move_down(&mut self) -> Option<&mut Self>;
    fn move_left(&mut self) -> Option<&mut Self>;
    fn move_right(&mut self, pos: Position) -> Option<Position>;
    fn move_up(&mut self) -> Option<&mut Self>;
    fn index(&self) -> usize;
    fn row_content(&self, pos: Position) -> &[u8];
    fn set_row_content(&mut self, data: String) -> Result<&mut Self, Self::Error>;
}
