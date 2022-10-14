use std::error::Error;

mod argparse;
mod maps;
mod dex;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args = argparse::parse();
    dex::dump(args.pid, args.output_dir)?;

    Ok(())
}
