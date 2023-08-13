use std::{fs, io};
use std::io::IoSliceMut;
use nix::sys::uio;
use nix::sys::uio::RemoteIoVec;
use nix::unistd::Pid;
use once_cell::sync::Lazy;
use regex::Regex;

static PATTERN_MEMORY_MAP: Lazy<Regex> = Lazy::new(|| {
    //               address    perms              pathname
    Regex::new(r"(\S+)-(\S+) (\S+) \S+ \S+ \S+\s+(\S.*)?").unwrap()
});

pub struct MemoryMap {
    pub address: (usize, usize),
    pub perms: String,
    pub filepath: Option<String>
}

impl MemoryMap {
    pub fn address(&self) -> usize {
        self.address.0
    }

    pub fn size(&self) -> usize {
        self.address.1 - self.address.0
    }

    pub fn name(&self) -> String {
        if let Some(name) = &self.filepath {
            name.to_string()
        } else {
            String::from("[anonymous memory]")
        }
    }
}

pub struct RemoteMemory {
    pid: i32
}

impl RemoteMemory {
    pub fn new(pid: i32) -> Self {
        Self { pid }
    }

    pub fn read_maps(&self) -> io::Result<Vec<MemoryMap>> {
        let maps_str = fs::read_to_string(format!("/proc/{}/maps", self.pid))?;

        macro_rules! next_address {
            ($iter:ident) => {
                usize::from_str_radix($iter.next().unwrap().unwrap(), 16).unwrap()
            };
        }

        macro_rules! next_string {
            ($iter:ident) => {
                $iter.next().unwrap().map(|it| it.to_string())
            };
        }

        Ok(
            maps_str.lines()
                .map(|line| {
                    let captures = PATTERN_MEMORY_MAP.captures(line).take().unwrap();
                    let mut captures = captures.iter().skip(1).map(|it| it.map(|it| it.as_str()));
                    MemoryMap {
                        address: (next_address!(captures), next_address!(captures)),
                        perms: next_string!(captures).unwrap(),
                        filepath: next_string!(captures)
                    }
                })
                .collect()
        )
    }

    pub fn read_memory(&self, map: &MemoryMap, buffer: &mut [u8]) -> io::Result<()> {
        let iov_local = IoSliceMut::new(buffer);
        let iov_remote = RemoteIoVec {
            base: map.address(),
            len: map.size(),
        };
        uio::process_vm_readv(Pid::from_raw(self.pid), &mut [iov_local], &[iov_remote])?;
        Ok(())
    }
}
