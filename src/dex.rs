use std::{fs, io, mem, ptr};
use std::fs::File;
use std::io::Write;

use lazy_static::lazy_static;
use regex::bytes::Regex;

use crate::argparse::Args;
use crate::maps::{Memory, MemoryMap};

lazy_static! {
    static ref MAGIC: Regex = Regex::new("dex\n\\d{3}\0").unwrap();
}

// https://source.android.com/docs/core/runtime/dex-format?hl=zh-cn#items
// https://android.googlesource.com/platform/art/+/master/libdexfile/dex/dex_file.h
#[repr(C, packed)]
struct DexHeader {
    magic: [u8; 8],
    checksum: u32,
    signature: [u8; 20],
    file_size:  u32,
    header_size: u32,  // = 0x70
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
    data_off: u32
}

impl DexHeader {
    const SIZE: usize = mem::size_of::<DexHeader>();

    fn new(buffer: &[u8], block: &MemoryMap) -> Option<DexHeader> {
        let result: DexHeader = unsafe {
            ptr::read(buffer.as_ptr() as *const _)
        };

        if result.verify(block) {
            Some(result)
        } else {
            None
        }
    }

    fn verify(&self, block: &MemoryMap) -> bool {
        if !MAGIC.is_match(&self.magic) { return false; }

        if block.size() < self.file_size as usize { return false; }

        if self.header_size as usize != Self::SIZE { return false; }

        // https://source.android.com/docs/core/runtime/dex-format?hl=zh-cn#endian-constant
        if self.endian_tag != 0x12345678 && self.endian_tag != 0x78563412 { return false; }

        if self.type_ids_size > 65535 || self.proto_ids_size > 65535 { return false; }

        true
    }

    #[inline]
    fn dex_size(&self) -> usize {
        self.file_size as _
    }
}

pub fn dump(args: &Args) -> Result<(), io::Error> {
    if let Some(output_dir) = &args.output_dir {
        fs::create_dir_all(&output_dir)?;
    }

    let mut memory = Memory::new(args.pid)?;
    let mut mappings = Vec::new();

    for block in memory.get_maps()? {
        let mut buffer = [0; DexHeader::SIZE];
        if memory.read(&block, &mut buffer).is_err() { continue; }

        if let Some(header) = DexHeader::new(&buffer, &block) {
            let source = block.pathname.clone().unwrap_or_else(|| String::from("[anonymous memory]"));

            if let Some(output_dir) = &args.output_dir {
                let mut buffer = vec![0; header.dex_size()];
                memory.read(&block, &mut buffer[..])?;

                let output = format!("dumped-{}.dex", mappings.len());

                fs::write(output_dir.join(&output), buffer)?;
                println!("[*] dumped dex file at {:x}: {}", block.address.0, source);

                mappings.push((source, output));
            } else {
                println!("[*] found dex file at {:x}: [{}] {}", block.address.0, block.perms, source);
            }
        }
    }

    if let Some(output_dir) = &args.output_dir {
        let mut fp = File::create(output_dir.join("mappings.txt")).unwrap();
        for (source, output) in &mappings {
            fp.write_all(format!("{}: {}\n", output, source).as_bytes())?;
        }

        println!("[*] dumped {} dex file(s) to {:?}", mappings.len(), output_dir);
    }

    Ok(())
}
