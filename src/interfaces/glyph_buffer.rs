use crate::{Position, Result};

pub trait GlyphBuffer {
    type Error: std::error::Error;
    
    fn contents(&self) -> &[u8];
    fn delete_glyph(&mut self) -> Result<&mut Self, Self::Error>;
    fn insert_glyph(&mut self, glyph: char) -> Result<&mut Self, Self::Error>;
    fn move_down(&mut self) -> Result<&mut Self, Self::Error>;
    fn move_left(&mut self) -> Result<&mut Self, Self::Error>;
    fn move_right(&mut self) -> Result<&mut Self, Self::Error>;
    fn move_up(&mut self) -> Result<&mut Self, Self::Error>;
    fn pos(&self) -> Position;
    fn set_pos(&mut self, pos: Position) -> Result<&mut Self, Self::Error>;
}
