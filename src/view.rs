use crate::position_buffer::PositionBuffer;
use std::io::{Read, Write, stdout, stdin};
use termion;

#[derive(Debug, Eq, PartialEq)]
pub struct Screen {
    data: PositionBuffer,
}

impl Screen {
    pub fn render() {
        write!(stdout, {})
    }
}
