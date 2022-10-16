extern crate core;

use std::error::Error;

mod argparse;
mod dex;
mod maps;
mod utils;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn Error>> {
    println!("[*] Hunter version: {}", VERSION);
    dex::dump(&argparse::parse())?;
    Ok(())
}
