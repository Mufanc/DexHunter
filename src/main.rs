use std::error::Error;

mod argparse;
mod dex;
mod maps;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    dex::dump(&argparse::parse())?;
    Ok(())
}
