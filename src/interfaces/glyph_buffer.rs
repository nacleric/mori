use crate::{
    position::Position,
    Result,
    buffer::direction::Direction,
};

pub trait GlyphBuffer {
    type Error: std::error::Error;

    fn contents(&self) -> &[u8];
    fn delete_glyph(&mut self, direction: Direction) -> Option<char>;
    fn insert_glyph(&mut self, glyph: char) -> &mut Self;
    fn move_down(&mut self) -> Option<&mut Self>;
    fn move_left(&mut self) -> Option<&mut Self>;
    fn move_right(&mut self) -> Option<&mut Self>;
    fn move_up(&mut self) -> Option<&mut Self>;
    fn pos(&self) -> Position;
    fn set_contents(&mut self, data: String) -> Result<&mut Self, Self::Error>;
    fn set_pos(&mut self, pos: Position) -> Result<&mut Self, Self::Error>;
}
