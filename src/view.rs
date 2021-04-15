#[cfg(test)]
mod unit_tests;

use termion::{self, raw::RawTerminal};
use termion::raw::IntoRawMode;

use std::io::Stdout;
use std::io::{stdin, stdout, Write};

use crate::{
    consts::{HEIGHT, WIDTH},
    interfaces::{View, ViewBuffer},
};

#[derive(Clone, Debug, PartialEq)]
pub struct MockTerminalBuffer {
    data: [[Option<char>; WIDTH]; HEIGHT],
}

impl MockTerminalBuffer {
    pub fn new() -> Self {
        Self {
            data: [[None; WIDTH]; HEIGHT],
        }
    }
}

impl ViewBuffer for MockTerminalBuffer {
    fn clear(&mut self) {
        self.data = [[Some(' '); WIDTH]; HEIGHT];
    }

    fn contents(&self) -> [[Option<char>; WIDTH]; HEIGHT] {
        self.data
    }
}
pub struct TerminalBuffer {
    data: [[Option<char>; WIDTH]; HEIGHT],
}

impl ViewBuffer for TerminalBuffer {
    fn clear(&mut self) {
        unimplemented!()
    }

    fn contents(&self) -> [[Option<char>; WIDTH]; HEIGHT] {
        unimplemented!()
    }
}

pub struct Terminal<B> {
    buffer: B,
    // input:
    output: RawTerminal<Stdout>,
}

impl<B: ViewBuffer> Terminal<B>
where
    B: Clone,
{
    pub fn new(buffer: B) -> Self {
        let stdout = stdout().into_raw_mode().unwrap();
        Self {
            buffer,
            output: stdout,
        }
    }

    pub fn buffer(&self) -> &B {
        let buf = &self.buffer;
        buf
    }
}

impl<B: ViewBuffer> View for Terminal<B> {
    fn clear(&mut self) {
        write!(self.output, "{}", termion::clear::All).unwrap();
        self.output.flush().unwrap();
    }

    fn print(&mut self) {
        unimplemented!();
    }
}
