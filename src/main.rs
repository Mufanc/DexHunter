mod argparse;
mod dump;
mod memory;
mod dex;

fn main() {
    if let Err(err) = dump::dump_dex_files(&argparse::parse()) {
        println!("[!] Error: {:?}", err)
    }
}
