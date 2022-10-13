use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};

use lazy_static::lazy_static;
use regex::Regex;

use crate::maps::Maps;

lazy_static! {
    static ref PATTERN: Regex = Regex::new("dex\n\\d{3}\0").unwrap();
}

// https://source.android.com/docs/core/runtime/dex-format?hl=zh-cn#items
pub fn dump(pid: i32) -> Result<(), io::Error> {
    let maps = Maps::new(pid)?;
    let mut memory = File::open(format!("/proc/{}/mem", maps.pid))?;

    maps.maps.iter().for_each(|it| {
        memory.seek(SeekFrom::Start(it.address.0)).unwrap();
        let mut buffer = [0; 8];
        if let Ok(_) = memory.read(&mut buffer) {
            let magic = buffer.iter().map(|it| *it as char).collect::<String>();
            if PATTERN.is_match(&magic) {
                println!("found dex header {:?} in {:?}", magic, it.pathname);
            }
        } else {
            eprintln!("error at: {:?}", it.pathname);
        }
    });

    Ok(())
}
