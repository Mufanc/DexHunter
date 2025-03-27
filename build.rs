use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn exec(args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = String::from_utf8(Command::new(args[0]).args(&args[1..]).output()?.stdout)?;
    Ok(String::from(output.trim()))
}

fn main() -> Result<(), Box<dyn Error>> {
    let output_dir = PathBuf::from(env::var("OUT_DIR")?);

    let mut version_file = File::create(output_dir.join("VERSION"))?;

    write!(
        version_file,
        "{}.r{}.{}",
        env::var("CARGO_PKG_VERSION")?,
        exec(&["git", "rev-list", "--count", "HEAD"])?,
        exec(&["git", "rev-parse", "--short", "HEAD"])?
    )?;

    Ok(())
}
