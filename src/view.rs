#[cfg(test)]
mod unit_tests;

pub mod mock_terminal; // TODO: change this back to private
pub mod termion_adapter;

use crate::{
    consts::{HEIGHT, WIDTH},
    interfaces::{TtyControl, View},
};
use std::io::{stdout, Stdout, Write};

pub struct Terminal<TC: TtyControl> {
    tty_control: TC,
    // input:
    output: Stdout,
}

impl<TC: TtyControl> Terminal<TC> {
    pub fn new(tty_control: TC) -> Self {
        let stdout = stdout();
        Self {
            tty_control,
            output: stdout,
        }
    }
}

impl<TC: TtyControl> View for Terminal<TC> {
    fn clear(&mut self) -> Result<(), std::io::Error> {
        match write!(self.output, "{}", termion::clear::All) {
            Ok(_) => self.output.flush(),
            Err(e) => panic!("Problem writing to screen: {:?}", e),
        }
    }

    fn print(&mut self) {
        unimplemented!();
    }
}
