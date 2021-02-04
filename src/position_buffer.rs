#[cfg(test)]
mod unit_tests;

use crate::{
    cursor_position::CursorPosition, interfaces::Buffer,
    utf8_buffer::Utf8Buffer,
};
use std::cmp::min;

#[derive(Debug, Eq, PartialEq)]
pub struct PositionBuffer {
    buffer: Utf8Buffer,
    position: CursorPosition,
}

impl PositionBuffer {
    pub fn new(buffer: Utf8Buffer, position: CursorPosition) -> Self {
        Self { buffer, position }
    }

    pub fn move_down(&mut self) -> &mut Self {
        let (col, row) = self.position.as_tuple();
        let max_row = self.buffer.row_count() - 1;

        let new_pos: CursorPosition;
        if row == max_row {
            new_pos = self.position;
        } else {
            let new_row = row + 1;
            new_pos = CursorPosition::new(
                min(self.buffer.col_count(new_row), col),
                new_row,
            );
        }
        self.position = new_pos;
        self
    }

    pub fn move_left(&self) {
        unimplemented!()
    }

    pub fn move_right(&self) {
        unimplemented!()
    }

    pub fn move_up(&self) {
        unimplemented!()
    }
}

impl Default for PositionBuffer {
    fn default() -> Self {
        Self {
            buffer: Utf8Buffer::default(),
            position: CursorPosition::default(),
        }
    }
}
