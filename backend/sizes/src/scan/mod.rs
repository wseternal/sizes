use std::path::{Path, PathBuf};
use std::io::ErrorKind;
use std::os::macos::fs::MetadataExt;
use std::ops::AddAssign;
use rocket::serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use crate::db::dirstat;
use crate::db::dirstat::DirStat;
use crate::kvstore::KvStore;
use crate::unix;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DirScanOverview {
    pub dirs: u64,
    pub files: u64,
    pub blocks: u64,
    pub is_cached: bool,
}

impl DirScanOverview {
    pub fn new() -> Self {
        Self {
            dirs: 0,
            files: 0,
            blocks: 0,
            is_cached: false,
        }
    }
}

impl AddAssign<&Self> for DirScanOverview {
    fn add_assign(&mut self, o: &Self) {
        *self = Self {
            is_cached: self.is_cached,
            dirs: self.dirs + o.dirs,
            files: self.files + o.files,
            blocks: self.blocks + o.blocks,
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirScanResult {
    pub cached: DirScanOverview,
    pub scanned: DirScanOverview,
    // in unit of second
    pub spent: u64,
    pub ongoing: bool
}

impl Display for DirScanResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "scanned: {:?}, cached: {:?}", self.scanned, self.cached)
    }
}

impl DirScanResult {
    pub fn new() -> Self {
        let mut obj = Self {
            cached: DirScanOverview::new(),
            scanned: DirScanOverview::new(),
            spent: 0,
            ongoing: true
        };
        obj.cached.is_cached = true;
        obj
    }
}

pub fn scan_one_dir(
    db: &impl KvStore,
    path: &Path
) -> (DirStat, bool) {
    let mut dir_stat = dirstat::get_dir_stat(db, path)
        .unwrap_or(DirStat::new(path));

    let Ok(dir_meta) = path.metadata() else {
        eprintln!("read dir meta failed on path {:?}", path);
        return (dir_stat, true);
    };

    if dir_stat.mtime == dir_meta.st_mtime() {
        return (dir_stat, true);
    }
    let entries = match path.read_dir() {
        Ok(entries) => entries,
        Err(e) => {
            if e.kind() != ErrorKind::PermissionDenied {
                eprintln!("read_dir failed on path {:?} {:?}", path, e);
            }
            return (dir_stat, true);
        }
    };
    dir_stat.sub_dirs.clear();

    for entry in entries {
        let Ok(entry) = entry else { continue };
        let Ok(meta) = entry.metadata() else { continue };

        if !(meta.is_dir() || meta.is_file()) {
            continue;
        }

        if meta.is_file() {
            dir_stat.file_num += 1;
            // use st_blocks instead of st_size() to deal with spare file properly
            dir_stat.blocks += meta.st_blocks();
            continue;
        }
        dir_stat.subdir_num += 1;
        dir_stat.sub_dirs.push(PathBuf::from(entry.file_name()));
    }
    dir_stat.mtime = dir_meta.st_mtime();
    dir_stat.ts = unix();

    if let Err(err) = dirstat::save_dir_stat(db, path, &dir_stat) {
        eprintln!("save dir stat for {:?} failed, {}", path, err);
    }
    (dir_stat, false)
}