#[cfg(test)]
mod unit_tests;

use crate::{
    buffer::{self, Buffer},
    interfaces::{Bufferable, Positionable},
    position::Position,
};

#[derive(Debug, Eq, PartialEq)]
pub struct PositionBuffer<B: Bufferable, P: Positionable> {
    buffer: B,
    position: P,
}

impl<B, P> PositionBuffer<B, P>
where
    B: Bufferable,
    P: Positionable,
{
    pub fn new(buffer: B, position: P) -> Self {
        PositionBuffer {
            buffer: buffer,
            position: position,
        }
    }

    fn buf(self, buffer: B) -> B {
        self.buffer
    }

    fn pos(self, position: P) -> P {
        self.position
    }
}

impl<B: Bufferable, P: Positionable> Positionable for PositionBuffer<B, P> {
    fn move_down(&self, pos: Position) -> Position {
        unimplemented!()
    }
    fn move_left(&self, pos: Position) -> Position {
        unimplemented!()
    }
    fn move_right(&self, pos: Position) -> Position {
        unimplemented!()
    }
    fn move_up(&self, pos: Position) -> Position {
        unimplemented!()
    }
}
