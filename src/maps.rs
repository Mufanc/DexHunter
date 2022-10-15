use std::{fs, io};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use lazy_static::lazy_static;
use regex::Regex;

use crate::utils;

lazy_static! {
    //                                       address     perms               pathname
    static ref PATTERN: Regex = Regex::new(r"(\S+)-(\S+) (\S+) \S+ \S+ \S+\s+(\S.*)?").unwrap();
}

#[derive(Debug)]
pub struct MemoryMap {
    pub address: (u64, u64),
    pub perms: String,
    pub pathname: Option<String>,
}

impl MemoryMap {
    pub fn size(&self) -> usize {
        (self.address.1 - self.address.0) as _
    }

    pub fn start(&self) -> usize {
        self.address.0 as _
    }
}

#[derive(Debug)]
pub struct Memory {
    pub pid: i32,
    memory: File,
}

impl Memory {
    pub fn new(pid: i32) -> Result<Self, io::Error> {
        Ok(Self {
            pid,
            memory: File::open(format!("/proc/{}/mem", pid))
                .map_err(utils::inspect("failed to open process memory!"))?,
        })
    }

    pub fn get_maps(&self) -> Result<Vec<MemoryMap>, io::Error> {
        macro_rules! next_address {
            ($iter:ident) => {
                u64::from_str_radix($iter.next().unwrap().unwrap(), 16).unwrap()
            };
        }

        macro_rules! next_string {
            ($iter:ident) => {
                $iter.next().unwrap().map(|it| it.to_string())
            };
        }

        let maps = fs::read_to_string(format!("/proc/{}/maps", self.pid))
            .map_err(utils::inspect("failed to read process maps!"))?;

        Ok(maps
            .lines()
            .map(|it| {
                let iter = PATTERN.captures(it).take().unwrap();
                let mut iter = iter.iter().skip(1).map(|it| it.map(|it| it.as_str()));

                MemoryMap {
                    address: (next_address!(iter), next_address!(iter)),
                    perms: next_string!(iter).unwrap(),
                    pathname: next_string!(iter),
                }
            })
            .collect())
    }

    pub fn read(&mut self, block: &MemoryMap, buffer: &mut [u8]) -> io::Result<()> {
        self.memory.seek(SeekFrom::Start(block.address.0))?;
        self.memory.read_exact(buffer)
    }
}
