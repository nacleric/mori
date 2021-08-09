#[cfg(test)]
mod unit_tests;

pub mod mock_terminal; // TODO: change this back to private
pub mod termion_adapter;

use crate::{
    consts::{HEIGHT, WIDTH},
    interfaces::{TtyControl, View},
    model::position_buffer::PositionBuffer,
};
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
