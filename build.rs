extern crate core;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn call(args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = String::from_utf8(Command::new(args[0]).args(&args[1..]).output()?.stdout)?;
    Ok(String::from(output.trim()))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut fp = File::create(Path::new(&env::var("OUT_DIR")?).join("VERSION"))?;
    write!(
        fp, "{}.r{}.{}",
        env::var("CARGO_PKG_VERSION")?,
        call(&["git", "rev-list", "--count", "HEAD"])?,
        call(&["git", "rev-parse", "--short", "HEAD"])?
    )?;
    Ok(())
}
