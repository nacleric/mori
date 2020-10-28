use crate::{Position, Result};

pub trait GlyphBuffer {
    type Error: std::error::Error;
    
    fn contents(&self) -> &[u8];
    fn delete_glyph(&mut self) -> Option<char>;
    fn insert_glyph(&mut self, glyph: char) -> &mut Self;
    fn move_down(&mut self) -> Option<&mut Self>;
    fn move_left(&mut self) -> Option<&mut Self>;
    fn move_right(&mut self) -> Option<&mut Self>;
    fn move_up(&mut self) -> Option<&mut Self>;
    fn pos(&self) -> Position;
    fn set_pos(&mut self, pos: Position) -> Result<&mut Self, Self::Error>;
}
