mod consts;
mod editor;
mod error;
mod interfaces;
mod model;
mod terminal;
mod tests;

use crate::{editor::Editor, error::Result};

fn main() -> Result<()> {
    Editor::default().run();
    Ok(())
}
