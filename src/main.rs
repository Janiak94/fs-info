use std::{
    fs::{metadata, read_dir, File},
    os::unix::fs::MetadataExt,
    path::Path,
};

fn get_size(path: impl AsRef<Path>) -> u64 {
    let metadata = metadata(&path).expect("Could not read metadata for {path}");
    if metadata.is_dir() {
        metadata.size()
            + read_dir(path)
                .expect("Could not read dir {path}")
                .map(|entry| {
                    let entry = entry.expect("Could not create entry");
                    get_size(entry.path())
                })
                .sum::<u64>()
    } else {
        metadata.size()
    }
}
struct FileSystemInfo {
    size: usize,
}

impl FileSystemInfo {
    fn new() -> Self {
        Self { size: 0 }
    }
}

struct FileSystem<P> {
    path: P,
    fs_info: FileSystemInfo,
}

impl<P> FileSystem<P>
where
    P: AsRef<Path>,
{
    fn new(path: P) -> Self {
        FileSystem {
            path: path,
            fs_info: FileSystemInfo::new(),
        }
    }

    fn stat(&self) -> u64 {
        get_size(&self.path)
    }
}

fn main() {
    // let file_systems = vec![env!("HOME")];
    let file_systems = vec![env!("PWD")];

    for fs in file_systems.iter() {
        let size = FileSystem::new(fs).stat();
        print!("{size}");
    }
}
