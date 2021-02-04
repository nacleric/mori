#[cfg(test)]
mod unit_tests;

use crate::{
    cursor_position::CursorPosition,
    interfaces::{Buffer, Position},
};

#[derive(Debug, Eq, PartialEq)]
pub struct PositionBuffer<B: Buffer> {
    buffer: B,
    position: CursorPosition,
}

impl<B> PositionBuffer<B>
where
    B: Buffer,
{
    pub fn new(buffer: B, position: CursorPosition) -> Self {
        Self { buffer, position }
    }
}

impl<B: Buffer> Position for PositionBuffer<B> {
    fn move_down(&mut self) -> &mut Self {
        let (col, row) = self.position.as_tuple();
        let lower_bound = self.buffer.row_count() - 1;

        let pos: CursorPosition;
        if row == lower_bound {
            pos = CursorPosition::new(col, row);
        } else {
            let eol_next_row = self.buffer.col_count(row + 1);
            if col > eol_next_row {
                pos = CursorPosition::new(eol_next_row, row + 1);
            } else {
                pos = CursorPosition::new(col, row + 1);
            }
        }
        self.position = pos;
        self
    }

    fn move_left(&self) {
        unimplemented!()
    }

    fn move_right(&self) {
        unimplemented!()
    }

    fn move_up(&self) {
        unimplemented!()
    }
}

impl<B> Default for PositionBuffer<B>
where
    B: Buffer + Default,
{
    fn default() -> Self {
        Self {
            buffer: B::default(),
            position: CursorPosition::default(),
        }
    }
}
