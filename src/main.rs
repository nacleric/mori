#![allow(dead_code)]
mod cli_args;
mod error;
mod position;
mod position_buffer;

use cli_args::CliArgs;
// use std::io;
use crate::buffer::*;
use structopt::StructOpt;

pub mod buffer;
pub mod interfaces;
pub use {
    error::{Error, Result},
    position::Position,
    position_buffer::PositionBuffer,
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
