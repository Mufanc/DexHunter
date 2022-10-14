use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(version, about)]
pub struct Args {
    #[clap(short, long, help="Process id of target application")]
    pub pid: i32,

    #[clap(help="Directory to save dumped dex files")]
    pub output_dir: PathBuf
}

pub fn parse() -> Args {
    Args::parse()
}
