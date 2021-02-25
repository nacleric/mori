#![allow(dead_code)]
mod cli_args;
mod consts;
mod cursor_position;
mod error;
mod position_buffer;
mod utf8_buffer;
mod view;

use cli_args::CliArgs;
// use std::io;
use crate::utf8_buffer::Utf8Buffer;
use structopt::StructOpt;

pub mod interfaces;
pub use error::{Error, Result};

// TODO: Read filepath and insert row_content into buffer
fn main() -> Result<()> {
    let filepath = CliArgs::from_args();
    // TODO: remove underscores eventually
    let _data = std::fs::read_to_string(filepath.path)?;
    let mut _buf = Utf8Buffer::new();

    // initialize_buffer(data)
    unimplemented!()
}
