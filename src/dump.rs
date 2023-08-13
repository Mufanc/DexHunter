use std::fs;
use std::process::Command;
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::bytes::Regex;

use crate::argparse::Args;
use crate::dex::MemoryDex;
use crate::memory::RemoteMemory;

const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

static PATTERN_TOP_ACTIVITY: Lazy<Regex> = Lazy::new(|| {
    Regex::new("ACTIVITY (\\S+) .*pid=(\\d+)\n.*\n.*?mResumed=true").unwrap()
});


pub fn dump_dex_files(args: &Args) -> anyhow::Result<()> {
    println!("[*] DexHunter version: {}", VERSION);

    if let Some(output_dir) = &args.output_dir {
        fs::create_dir_all(output_dir)?;
        if output_dir.read_dir()?.next().is_some() {
            println!("[!] Target directory is not empty, abort!");
            return Ok(())
        }
    }

    let mut mappings = vec![];
    let mut top_activity: Option<String> = None;

    let memory = RemoteMemory::new(match args.pid {
        Some(pid) => pid,
        None => {
            let output = Command::new("/system/bin/dumpsys")
                .args(["activity", "top"])
                .output()?
                .stdout;

            let captures = PATTERN_TOP_ACTIVITY
                .captures(&output[..])
                .ok_or(anyhow::format_err!("failed to get pid of the top activity"))?;

            top_activity = Some(String::from_utf8(captures[1].to_vec())?);

            i32::from_str(&String::from_utf8(captures[2].to_vec())?)?
        }
    });

    if let Some(name) = top_activity {
        println!("[*] Top activity: {}", name)
    }

    for map in memory.read_maps()? {
        if let Ok(dex) = MemoryDex::new(&memory, &map) {
            if !dex.is_valid() {
                if args.verbose {
                    println!("[*] Skipped: {}", map.name())
                }
                continue
            }

            if let Some(output_dir) = &args.output_dir {
                let output_file = format!("dumped-{}.dex", mappings.len());

                dex.dump(&output_dir.join(&output_file))?;
                mappings.push((output_file, map.name()));

                println!("[*] Dumped dex at {:x}: {}", map.address(), map.name())
            } else {
                println!("[*] Found dex at {:x} ({}): {}", map.address(), map.perms, map.name())
            }
        }
    }

    if let Some(output_dir) = &args.output_dir {
        fs::write(
            output_dir.join("mappings.txt"),
            mappings.iter()
                .map(|(source, output)| format!("{} {}", source, output))
                .collect::<Vec<_>>()
                .join("\n")
        )?;

        println!("[*] Dumped {} dex file(s) to {:?}", mappings.len(), output_dir);
    }

    Ok(())
}
