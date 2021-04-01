#[cfg(test)]
mod unit_tests;

use crate::{
    consts::{HEIGHT, WIDTH},
    interfaces::{Terminal, ViewBuffer},
};

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
        self.buffer = [[None; WIDTH]; HEIGHT];
    }
}

pub struct View<B> {
    buffer: B,
}

impl<B: ViewBuffer> View<B> {
    pub fn new(buffer: B) -> Self {
        Self { buffer }
    }

//     pub fn contents(&self) -> &B {
//         let buf = &self.buffer;
//         buf
//     }
}

impl<B: ViewBuffer> Terminal for View<B> {
    fn clear(&mut self) {
        self.buffer.clear()
    }
}
