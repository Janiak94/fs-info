#[allow(dead_code)]
mod fs;
use anyhow::Result;
use std::{path::PathBuf, str::FromStr};

const FILE_SYSTEMS: [&str; 1] = ["/"];

fn main() -> Result<()> {
    for file_system in FILE_SYSTEMS.iter() {
        let fs_info = fs::FsInfo::build(&PathBuf::from_str(file_system).unwrap());
        println!("{}", fs_info.unwrap());
    }
    Ok(())
}
