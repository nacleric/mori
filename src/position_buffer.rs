#[cfg(test)]
mod unit_tests;

use crate::{
    cursor_position::CursorPosition,
    interfaces::{Buffer, Position},
};

#[derive(Debug, Eq, PartialEq)]
pub struct PositionBuffer<B: Buffer, P: Position> {
    buffer: B,
    position: P,
}

impl<B, P> PositionBuffer<B, P>
where
    B: Buffer,
    P: Position + Copy,
{
    pub fn new(buffer: B, position: P) -> Self {
        Self { buffer, position }
    }
}

impl<B: Buffer, P: Position> Position for PositionBuffer<B, P> {
    fn move_down(&self, pos: CursorPosition) -> CursorPosition {
        unimplemented!()
    }
    fn move_left(&self, pos: CursorPosition) -> CursorPosition {
        unimplemented!()
    }
    fn move_right(&self, pos: CursorPosition) -> CursorPosition {
        unimplemented!()
    }
    fn move_up(&self, pos: CursorPosition) -> CursorPosition {
        unimplemented!()
    }
}
