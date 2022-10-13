use std::error::Error;

mod argparse;
mod maps;
mod dump;

fn main() -> Result<(), Box<dyn Error>> {
    let args = argparse::parse();

    dump::dump(args.pid)?;

    Ok(())
}
