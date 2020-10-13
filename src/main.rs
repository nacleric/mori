mod editor;
mod cli_args;
mod error;
use cli_args::CliArgs;
pub use error::Error;
use structopt::StructOpt;

fn main() {
    let args = CliArgs::from_args();
    dbg!(args);
}
