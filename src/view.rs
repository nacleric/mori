#[cfg(test)]
mod unit_tests;

pub mod mock_terminal; // TODO: change this back to private
pub mod termion_adapter;

use crate::{
    consts::{HEIGHT, WIDTH},
    interfaces::View,
};
use std::io::{stdout, Stdout, Write};

// We can handle inputs later, need output for tests
pub struct Terminal {
    // input:
    output: Stdout,
}

impl Terminal {
    pub fn new() -> Self {
        let stdout = stdout();
        Self {
            output: stdout,
        }
    }
}

impl View for Terminal {
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
