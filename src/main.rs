pub use error::{Error, Result};

// use cli_args::CliArgs;
use crate::{
    interfaces::{TtyControl, View},
    model::{
        cursor_position::CursorPosition, position_buffer::PositionBuffer, utf8_buffer::Utf8Buffer,
    },
    view::{mock_terminal::MockTerminalView, termion_adapter::TermionAdapter, Terminal},
};

mod consts;
mod error;
mod model;
mod view;

pub mod interfaces;

fn main() -> Result<(), std::io::Error> {
    // let utf8_buffer = PositionBuffer::new(Utf8Buffer::new(), CursorPosition::default());
    // TODO: Utf8Buffer will read from a file instead of new()
    let mut terminal = TermionAdapter::new();
    terminal.clear();
    Ok(())
}
