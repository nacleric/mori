mod test;
mod editor;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct CliArgs {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let args = CliArgs::from_args();
    dbg!(args);
}