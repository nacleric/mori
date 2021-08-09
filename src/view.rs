#[cfg(test)]
mod unit_tests;

pub mod mock_terminal; // TODO: change this back to private
pub mod termion_adapter;

use crate::{
    consts::{HEIGHT, WIDTH},
    interfaces::{TtyControl, View},
};
use std::io::{Stdin, Stdout, Write, stdin, stdout};

#[derive(Debug)]
// We can handle inputs later, need output for tests
pub struct Terminal {
    // input:
    input: Stdin,
    output: Stdout,
}

impl Terminal {
    pub fn new() -> Self {
        let stdin = stdin();
        let stdout = stdout();
        Self {
            input: stdin,
            output: stdout,
        }
    }
}

// OLD CODE REDO
// impl<TC: TtyControl> Terminal<TC: TtyControl> {
//     pub fn new(tty_control: TC) -> Self {
//         let stdout = stdout();
//         Self {
//             tty_control,
//             output: stdout,
//         }
//     }
// }

// impl View for Terminal {
//     fn clear(&mut self) -> Result<(), std::io::Error> {
//         match write!(self.output, "{}", termion::clear::All) {
//             Ok(_) => self.output.flush(),
//             Err(e) => panic!("Problem writing to screen: {:?}", e),
//         }
//     }

//     fn print(&mut self) {
//         unimplemented!();
//     }
// }
