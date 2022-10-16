use std::{fs, mem, ptr};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::bytes::Regex;

use crate::argparse::Args;
use crate::maps::{Memory, MemoryMap};

const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

lazy_static! {
    static ref DEX_MAGIC: Regex = Regex::new("dex\n\\d{3}\0").unwrap();
    static ref TOP_ACTIVITY: Regex =
        Regex::new("ACTIVITY.*pid=(\\d+)\n.*\n\\s+mResumed=true").unwrap();
}

// https://source.android.com/docs/core/runtime/dex-format?hl=zh-cn#items
// https://android.googlesource.com/platform/art/+/master/libdexfile/dex/dex_file.h
#[repr(C, packed)]
struct DexHeader {
    magic: [u8; 8],
    checksum: u32,
    signature: [u8; 20],
    file_size: u32,
    header_size: u32,
    endian_tag: u32,
    link_size: u32,
    link_off: u32,
    map_off: u32,
    string_ids_size: u32,
    string_ids_off: u32,
    type_ids_size: u32,
    type_ids_off: u32,
    proto_ids_size: u32,
    proto_ids_off: u32,
    field_ids_size: u32,
    field_ids_off: u32,
    method_ids_size: u32,
    method_ids_off: u32,
    class_ids_size: u32,
    class_ids_off: u32,
    data_size: u32,
    data_off: u32,
}

impl DexHeader {
    const SIZE: usize = mem::size_of::<DexHeader>();

    fn new(buffer: &[u8], block: &MemoryMap) -> Option<DexHeader> {
        let result: DexHeader = unsafe { ptr::read(buffer.as_ptr() as *const _) };

        if result.verify(block) {
            Some(result)
        } else {
            None
        }
    }

    fn verify(&self, block: &MemoryMap) -> bool {
        if !DEX_MAGIC.is_match(&self.magic) {
            return false;
        }

        if block.size() < self.file_size as usize {
            return false;
        }

        if self.header_size as usize != Self::SIZE {
            return false;
        }

        // https://source.android.com/docs/core/runtime/dex-format?hl=zh-cn#endian-constant
        if self.endian_tag != 0x12345678 && self.endian_tag != 0x78563412 {
            return false;
        }

        if self.type_ids_size > 65535 || self.proto_ids_size > 65535 {
            return false;
        }

        true
    }

    fn dex_size(&self) -> usize {
        self.file_size as _
    }
}

pub fn dump(args: &Args) -> Result<(), Box<dyn Error>> {
    if let Some(output_dir) = &args.output_dir {
        fs::create_dir_all(&output_dir)?;
    }

    let mut mappings = Vec::new();
    let mut memory = Memory::new(match args.pid {
        Some(pid) => pid,
        None => {
            let output = Command::new("/system/bin/dumpsys")
                .args(["activity", "top"])
                .output()?
                .stdout;
            let capture = TOP_ACTIVITY
                .captures(&output[..])
                .ok_or("failed to get the pid of the top activity")?;
            i32::from_str(&String::from_utf8(capture[1].to_vec())?)?
        }
    })?;

    println!("[*] Hunter version: {}", VERSION);

    for block in memory.get_maps()? {
        let mut buffer = [0; DexHeader::SIZE];
        if memory.read(&block, &mut buffer).is_err() {
            continue;
        }

        if let Some(header) = DexHeader::new(&buffer, &block) {
            let source = block
                .pathname
                .clone()
                .unwrap_or_else(|| String::from("[anonymous memory]"));

            if let Some(output_dir) = &args.output_dir {
                let mut buffer = vec![0; header.dex_size()];
                memory.read(&block, &mut buffer[..])?;

                let output = format!("dumped-{}.dex", mappings.len());

                fs::write(output_dir.join(&output), buffer)?;
                println!("[*] Dumped dex file at {:x}: {}", block.start(), source);

                mappings.push((source, output));
            } else {
                println!(
                    "[*] Dex file found at {:x}: [{}] {}",
                    block.start(),
                    block.perms,
                    source
                );
            }
        }
    }

    if let Some(output_dir) = &args.output_dir {
        let mut fp = File::create(output_dir.join("mappings.txt")).unwrap();
        for (source, output) in &mappings {
            fp.write_all(format!("{}: {}\n", output, source).as_bytes())?;
        }

        println!(
            "[*] Dumped {} dex file(s) to {:?}",
            mappings.len(),
            output_dir
        );
    }

    Ok(())
}
