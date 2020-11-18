#![allow(dead_code)]
mod cli_args;
mod error;
mod position;
pub mod interfaces;
pub mod buffer;

use cli_args::CliArgs;
pub use {error::{Error, Result}, position::Position};
use structopt::StructOpt;

fn main() {
    let args = CliArgs::from_args();
    dbg!(args);
}
