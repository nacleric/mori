use crate::interfaces::Buffer;
// use crate::{cursor_position::CursorPosition, utf8_buffer::direction::Direction, Result};
pub trait GraphemeBuffer: Buffer {
    // type Error: std::error::Error;
    type Column;

    // fn content(&self) -> Vec<String>;
    // fn delete_grapheme(
    //     &mut self,
    //     direction: Direction,
    //     col: usize,
    // ) -> (CursorPosition, Option<char>);
    // fn delete_graphemes(&mut self) -> (CursorPosition, Vec<String>);
    fn insert_grapheme(&mut self, col_index: usize, grapheme: char) -> Option<Self::Column>;
    // fn insert_graphemes<I: Iterator<Item = char>>(
    //     &mut self,
    //     pos: CursorPosition,
    //     graphemes: I,
    // ) -> CursorPosition;
    // fn index(&self, pos: CursorPosition) -> usize;
    // fn row_content(&self, pos: CursorPosition) -> &[u8];
    // fn set_row_content(
    //     &mut self,
    //     pos: CursorPosition,
    //     data: String,
    // ) -> Result<&mut Self, Self::Error>;
}
