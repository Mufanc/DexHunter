use std::{fs, io};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    //                                      address       perms                    pathname
    static ref PATTERN: Regex = Regex::new("(\\S+)-(\\S+) (\\S+) \\S+ \\S+ \\S+\\s+(\\S.*)?").unwrap();
}

#[derive(Debug)]
pub struct Block {
    pub address: (u64, u64),
    pub perms: String,
    pub pathname: Option<String>
}

#[derive(Debug)]
pub struct Maps {
    pub pid: i32,
    pub maps: Vec<Block>
}

impl Maps {
    pub fn new(pid: i32) -> Result<Self, io::Error> {
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

        let maps = fs::read_to_string(format!("/proc/{}/maps", pid))?;

        Ok(Self {
            pid,
            maps: maps.lines()
                .map(|it| {
                    let iter = PATTERN.captures(it).take().unwrap();
                    let mut iter = iter.iter().skip(1)
                        .map(|it| it.map(|it| it.as_str()));

                    Block {
                        address: (next_address!(iter), next_address!(iter)),
                        perms: next_string!(iter).unwrap(),
                        pathname: next_string!(iter)
                    }
                })
                .collect()
        })
    }
}
