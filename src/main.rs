#![allow(dead_code)]
mod cli_args;
mod consts;
mod cursor_position;
mod error;
mod position_buffer;
mod utf8_buffer;
mod view;

// use cli_args::CliArgs;
// use std::io;
use crate::view::{Terminal, MockTerminalBuffer};
// use structopt::StructOpt;

pub mod interfaces;
pub use error::{Error, Result};
use interfaces::View;

// TODO: Read filepath and insert row_content into buffer
fn main() {
    let mock_view = MockTerminalBuffer::new();
    let mut terminal = Terminal::new(mock_view);
    terminal.clear();
    // unimplemented!()
}
