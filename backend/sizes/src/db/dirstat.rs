use crate::db::TABLE_DIR_STAT;
use crate::kvstore::KvStore;
use crate::scan::DirScanOverview;
use crate::unix;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
pub struct DirStat {
    pub ts: u64,
    pub mtime: i64,
    pub subdir_num: u64,
    pub file_num: u64,
    pub blocks: u64,
    pub sub_dirs: Vec<PathBuf>,
    pub path: PathBuf
}

impl DirStat {
    pub fn new(path: &Path) -> Self {
        Self {
            path: PathBuf::from(path),
            ts: unix(),
            sub_dirs: Vec::new(),
            mtime: -1,
            subdir_num: 0,
            file_num: 0,
            blocks: 0,
        }
    }
}

pub fn save_dir_stat(
    db: &impl KvStore,
    path: &Path,
    stat: &DirStat,
) -> crate::Result<()> {
    db.set_json(TABLE_DIR_STAT, path.to_string_lossy(), stat)
}

pub fn get_dir_stat(db: &impl KvStore, path: &Path) -> Option<DirStat> {
    db.get_as(TABLE_DIR_STAT, path.to_string_lossy())
}

// sum(blocks) on the dir and all sub-dirs recursively
pub fn get_dir_stat_recursive(db: &impl KvStore, path: &Path) -> Option<DirScanOverview> {
    let mut overview = DirScanOverview::new();

    db.foreach(TABLE_DIR_STAT, path.to_string_lossy(), 0, |_,v| {
        let res: serde_json::Result<DirStat> = serde_json::from_str(v);
        if let Ok(stat) = res {
            overview.blocks += stat.blocks;
            overview.dirs += stat.subdir_num;
            overview.files += stat.file_num;
            overview.is_cached = false;
        }
    });

    Some(overview)
}
