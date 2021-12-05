#[cfg(test)]
mod unit_tests;

use crate::{
    model::position_buffer::PositionBuffer,
};
use termion::clear;

use std::io::{stdin, stdout, Stdin, Stdout, Write};

#[derive(Debug)]
pub struct Terminal {
    position_buffer: PositionBuffer,
    input: Stdin,
    output: Stdout,
}

impl Terminal {
    pub fn new(buffer: PositionBuffer) -> Self {
        let stdin = stdin();
        let stdout = stdout();
        Self {
            position_buffer: buffer,
            input: stdin,
            output: stdout,
        }
    }

    pub fn clear_screen(&mut self) -> Result<(), std::io::Error> {
        match write!(self.output, "{}", clear::All) {
            Ok(_) => self.output.flush(),
            Err(e) => panic!("Problem clearing the screen: {:?}", e),
        }
    }

    pub fn draw(&mut self) {
        unimplemented!()
    }

    pub fn print(&mut self) {
        unimplemented!()
    }

    pub fn render_frame(&mut self) {
        unimplemented!()
    }

    pub fn resize(&mut self) {
        unimplemented!()
    }

    pub fn set_color(&mut self) {
        unimplemented!()
    }
}

impl Default for Terminal {
    fn default() -> Self {
        let stdin = stdin();
        let stdout = stdout();
        Self {
            position_buffer: PositionBuffer::default(),
            input: stdin,
            output: stdout,
        }
    }
}
