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
pub struct MockTerminalView {
    data: [[Option<char>; WIDTH]; HEIGHT],
}

impl MockTerminalView {
    pub fn new() -> Self {
        Self {
            data: [[None; WIDTH]; HEIGHT],
        }
    }
}

impl ViewBuffer for MockTerminalView {
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

pub struct Terminal<View> {
    view: View,
    // input:
    output: RawTerminal<Stdout>,
}

impl<View: ViewBuffer> Terminal<View>
where
    View: Clone,
{
    pub fn new(view: View) -> Self {
        let stdout = stdout().into_raw_mode().unwrap();
        Self {
            view,
            output: stdout,
        }
    }

    pub fn view(&self) -> &View {
        let view = &self.view;
        view
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
