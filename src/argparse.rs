use clap::Parser;

#[derive(Parser)]
#[clap(version, about)]
pub struct Args {
    #[clap(short, long, help="Process id to dump dex files")]
    pub pid: i32
}

pub fn parse() -> Args {
    Args::parse()
}
