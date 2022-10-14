use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(version, about)]
pub struct Args {
    #[clap(short, long, help="Process id of target application")]
    pub pid: i32,

    #[clap(short, long="output-dir", value_names=&["DIR"], help="Directory to save dumped dex files")]
    pub output_dir: Option<PathBuf>
}

pub fn parse() -> Args {
    Args::parse()
}
