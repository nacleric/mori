#![allow(dead_code)]
pub mod buffer;
mod cli_args;
mod error;
pub mod interfaces;
mod position;

use cli_args::CliArgs;
use std::io;
use structopt::StructOpt;
pub use {
    error::{Error, Result},
    position::Position,
};
use crate::{
    buffer::*,
    interfaces::*,
};

// TODO: Read filepath and insert row_content into buffer
// Step 2: decouple it in a way that makes sense. Maybe put it into a folder called views
fn main() -> Result<()> {
    let filepath = CliArgs::from_args();
    let data = std::fs::read_to_string(filepath.path)?;
    let mut buf = Buffer::new();

    // initialize_buffer(data)
    unimplemented!()
}
