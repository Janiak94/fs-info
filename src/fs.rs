use anyhow::Result;

#[derive(Debug)]
struct FsSize {
    total: u64,
    available: u64,
}

impl From<nix::sys::statvfs::Statvfs> for FsSize {
    fn from(statvfs: nix::sys::statvfs::Statvfs) -> Self {
        let block_size = statvfs.fragment_size();
        FsSize {
            total: (statvfs.blocks() as u64) * block_size,
            available: (statvfs.blocks_available() as u64) * block_size,
        }
    }
}

fn format_bytes(n_bytes: u64) -> String {
    // TODO: Specify precision.
    const KB: f32 = 1024.0;
    const SUFFIXES: [&str; 4] = ["KB", "MB", "GB", "TB"];
    let exp = f32::floor(f32::ln(n_bytes as f32) / f32::ln(KB)) as usize;
    let suffix = SUFFIXES.get(exp - 1).expect("Exponent out of range");
    let prefix = (n_bytes as f32) / KB.powf(exp as f32);
    String::from_iter([prefix.to_string(), suffix.to_string()])
}

impl std::fmt::Display for FsSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let total = format_bytes(self.total);
        let available = format_bytes(self.available);
        write!(f, "Total: {}\nAvailable: {}", total, available)
    }
}

#[derive(Debug)]
struct FsInodes {
    total: u64,
    available: u64,
}

impl From<nix::sys::statvfs::Statvfs> for FsInodes {
    fn from(statvfs: nix::sys::statvfs::Statvfs) -> Self {
        FsInodes {
            total: statvfs.files() as u64,
            available: statvfs.files_available() as u64,
        }
    }
}

#[derive(Debug)]
pub struct FsInfo<P> {
    path: P,
    size: FsSize,
    inodes: FsInodes,
}

impl<P> FsInfo<P>
where
    P: nix::NixPath + Clone,
{
    pub fn build(path: &P) -> Result<Self> {
        let statvfs = nix::sys::statvfs::statvfs(path)?;

        let fs_size = FsSize::from(statvfs);
        let fs_inodes = FsInodes::from(statvfs);

        Ok(Self {
            path: path.clone(),
            size: fs_size,
            inodes: fs_inodes,
        })
    }
}

impl<P> std::fmt::Display for FsInfo<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.size)
    }
}
