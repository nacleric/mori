pub use error::{Error, Result};

// use cli_args::CliArgs;

use crate::{
    interfaces::View,
    view::{mock_terminal::MockTerminalView, Terminal, termion_adapter::TermionAdapter},
};

mod consts;
mod error;
mod model;
mod view;

pub mod interfaces;
// New
fn main() -> Result<(), std::io::Error> {
    let mock_view = MockTerminalView::new(); // TODO: This line is pretty useless, fix the api
    let termion = TermionAdapter::new();
    let mut terminal = Terminal::new(mock_view, termion);
    terminal.clear()?;
    Ok(())
}
