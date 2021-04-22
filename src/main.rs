#![allow(dead_code)]
mod cli_args;
mod consts;
mod cursor_position;
mod error;
mod position_buffer;
mod utf8_buffer;
mod view;

pub mod interfaces;
pub use error::{Error, Result};

// use cli_args::CliArgs;
// use std::io;
use crate::{
    interfaces::View,
    view::{mock_terminal::MockTerminalView, termion_wrapper::TermionWrapper, Terminal},
};
// use structopt::StructOpt;

// TODO: Read filepath and insert row_content into view
fn main() {
    let mock_view = MockTerminalView::new(); // TODO: This line is pretty useless, fix the api
    let ui_lib = TermionWrapper::new();
    let mut terminal = Terminal::new(mock_view, Some(ui_lib));
    terminal.clear();
}
