mod cli_args;
mod editor;
mod error;
mod position;
pub mod interfaces;
use cli_args::CliArgs;
pub use {error::{Error, Result}, position::Position};
use structopt::StructOpt;

fn main() {
    let args = CliArgs::from_args();
    dbg!(args);
}
