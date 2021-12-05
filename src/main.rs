mod consts;
mod editor;
mod error;
mod interfaces;
mod model;
mod view;

use crate::{
    editor::Editor,
    error::{Error, Result},
    model::{
        cursor_position::CursorPosition, position_buffer::PositionBuffer, utf8_buffer::Utf8Buffer,
    },
    view::Terminal, // might replace this
};

use std::io::{self, stdout};

fn main() -> Result<()> {
    let test_utf8_buffer = Utf8Buffer::from(vec![String::from("hello"), String::from("world")]);
    let main_buffer = PositionBuffer::new(test_utf8_buffer, CursorPosition::default());
    // TODO: Utf8Buffer will read from a file instead of new()
    let mut display = Terminal::default();
    display.clear_screen()?;

    let editor = Editor::default();
    editor.run();
    Ok(())
}
