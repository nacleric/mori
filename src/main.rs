#![allow(dead_code)]
pub mod buffer;
mod cli_args;
mod error;
pub mod interfaces;
mod position;

use cli_args::CliArgs;
// use std::io;
use crate::buffer::*;
use structopt::StructOpt;
pub use {
    error::{Error, Result},
    position::Position,
};

// TODO: Read filepath and insert row_content into buffer
// Step 2: decouple it in a way that makes sense. Maybe put it into a folder called views
fn main() -> Result<()> {
    let filepath = CliArgs::from_args();
    // TODO: remove underscores eventually
    let _data = std::fs::read_to_string(filepath.path)?;
    let mut _buf = Buffer::new();

    // initialize_buffer(data)
    unimplemented!()
}
