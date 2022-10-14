extern crate core;

use std::error::Error;

mod argparse;
mod maps;
mod dex;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    dex::dump(&argparse::parse())?;
    Ok(())
}
