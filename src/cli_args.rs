use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct CliArgs {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
