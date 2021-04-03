#[cfg(test)]
mod unit_tests;

use crate::{
    consts::{HEIGHT, WIDTH},
    interfaces::{Terminal, ViewBuffer},
};

#[derive(Clone, Debug, PartialEq)]
pub struct MockView {
    buffer: [[Option<char>; WIDTH]; HEIGHT],
}

impl MockView {
    pub fn new() -> Self {
        Self {
            buffer: [[None; WIDTH]; HEIGHT],
        }
    }
}

impl ViewBuffer for MockView {
    fn clear(&mut self) {
        self.buffer = [[Some(' '); WIDTH]; HEIGHT];
    }

    fn contents(&self) -> [[Option<char>; WIDTH]; HEIGHT] {
        self.buffer
    }
}

pub struct View<B> {
    buffer: B,
}

impl<B: ViewBuffer> View<B>
where
    B: Clone,
{
    pub fn new(buffer: B) -> Self {
        Self { buffer }
    }

    pub fn buffer(&self) -> B {
        let buf = self.buffer.clone();
        buf
    }
}

impl<B: ViewBuffer> Terminal for View<B> {
    fn clear(&mut self) {
        self.buffer.clear()
    }
}
