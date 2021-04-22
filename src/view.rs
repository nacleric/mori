#[cfg(test)]
mod unit_tests;

pub mod mock_terminal; // TODO: change this back to private
pub mod termion_adapter;

use crate::{
    consts::{HEIGHT, WIDTH},
    interfaces::{TtyControl, View, ViewBuffer},
};
use std::io::{stdout, Stdout, Write};

// TODO: Will have to be changed
pub struct TerminalBuffer {
    data: [[Option<char>; WIDTH]; HEIGHT],
}

impl ViewBuffer for TerminalBuffer {
    fn clear(&mut self) {
        unimplemented!()
    }
}

pub struct Terminal<VB, TC: TtyControl> {
    view_buffer: VB,
    tty_control: TC,
    // input:
    output: Stdout,
}

impl<VB, TC: TtyControl> Terminal<VB, TC>
where
    VB: ViewBuffer + Clone,
{
    pub fn new(view: VB, tty_control: TC) -> Self {
        let stdout = stdout();
        Self {
            view_buffer: view,
            tty_control,
            output: stdout,
        }
    }

    pub fn view_buffer(&self) -> &VB {
        &self.view_buffer
    }
}

impl<B: ViewBuffer, TC: TtyControl> View for Terminal<B, TC> {
    fn clear(&mut self) {
        write!(self.output, "{}", termion::clear::All).unwrap();
        self.output.flush().unwrap();
    }

    fn print(&mut self) {
        unimplemented!();
    }
}
