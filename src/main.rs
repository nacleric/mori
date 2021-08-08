pub use error::{Error, Result};

// use cli_args::CliArgs;
use crate::{
    interfaces::View,
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
    let mock_view = MockTerminalView::new(); // TODO: This line is pretty useless, fix the api
    let termion = TermionAdapter::new();
    // TODO: Utf8Buffer will read from a file instead of new()
    let utf8_buffer = PositionBuffer::new(Utf8Buffer::new(), CursorPosition::default());
    // let mut terminal = Terminal::new(mock_view, termion);
    // terminal.clear()?;
    Ok(())
}
