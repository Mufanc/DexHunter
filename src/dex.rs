use std::{fs, mem, ptr};
use std::path::PathBuf;
use once_cell::sync::Lazy;
use regex::bytes::Regex;
use crate::memory::{RemoteMemory, MemoryMap};

static PATTERN_DEX_MAGIC: Lazy<Regex> = Lazy::new(|| {
    Regex::new("dex\n\\d{3}\0").unwrap()
});

// https://source.android.com/docs/core/runtime/dex-format?hl=zh-cn#items
// https://android.googlesource.com/platform/art/+/master/libdexfile/dex/dex_file.h
#[repr(C, packed)]
struct Header {
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

impl Header {
    const SIZE: usize = mem::size_of::<Self>();

    fn new(buffer: &[u8]) -> Self {
        unsafe {
            ptr::read(buffer.as_ptr() as *const Header)
        }
    }
}

pub struct MemoryDex<'a> {
    memory: &'a RemoteMemory,
    map: &'a MemoryMap,
    header: Header,
}

impl<'a> MemoryDex<'a> {
    pub fn new(memory: &'a RemoteMemory, map: &'a MemoryMap) -> anyhow::Result<Self> {
        let mut buffer = [0; Header::SIZE];

        memory.read_memory(&map, &mut buffer)?;

        Ok(Self {
            memory, map,
            header: Header::new(&buffer),
        })
    }

    pub fn is_valid(&self) -> bool {
        if !PATTERN_DEX_MAGIC.is_match(&self.header.magic) {
            return false
        }

        if self.map.size() < self.header.file_size as usize {
            return false
        }

        // https://source.android.com/docs/core/runtime/dex-format?hl=zh-cn#endian-constant
        if self.header.endian_tag != 0x12345678 && self.header.endian_tag != 0x78563412 {
            return false;
        }

        if self.header.type_ids_size > 65535 || self.header.proto_ids_size > 65535 {
            return false;
        }

        return true
    }

    pub fn size(&self) -> usize {
        return self.header.file_size as usize
    }

    pub fn dump(&self, output_file: &PathBuf) -> anyhow::Result<()> {
        let mut buffer = vec![0; self.size()];

        self.memory.read_memory(&self.map, &mut buffer[..])?;
        fs::write(output_file, buffer)?;

        Ok(())
    }
}
