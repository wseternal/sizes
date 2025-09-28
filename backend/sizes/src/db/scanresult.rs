use std::path::Path;

use super::dirstat::DirStat;
use crate::db::TABLE_DIR_SCAN_RESULT;
use crate::kvstore::KvStore;
use crate::scan::DirScanResult;
use crate::unix;

pub fn save_dir_scan_result(
    db: &impl KvStore,
    path: &Path,
    result: &'static DirScanResult,
) -> crate::Result<()> {
    let key = format!("{},{}", path.to_string_lossy(), unix());
    db.set_json(TABLE_DIR_SCAN_RESULT, key, result)
}

pub fn get_dir_scan_result(
    db: &impl KvStore,
    path: &Path,
    limit: u32,
) -> crate::Result<Vec<DirScanResult>> {
    let mut results: Vec<DirScanResult> = Vec::new();

    db.foreach(TABLE_DIR_SCAN_RESULT, path.to_string_lossy(), limit, |_,v| {
        let elem: serde_json::Result<DirScanResult> = serde_json::from_str(v);
        if let Ok(result) = elem {
            results.push(result);
        }
    });
    Ok(results)
}

pub fn get_largest_dirs(
    db: &impl KvStore,
    min: u64,
    limit: u32,
    offset: u32
) -> crate::Result<Vec<DirStat>> {
    let mut ret: Vec<DirStat> = Vec::new();
    // TODO implement this endpoint
    Ok(ret)
}

pub fn get_last_dir_scan_result(db: &impl KvStore, path: &Path) -> Option<DirScanResult> {
    let Ok(results) = get_dir_scan_result(db, path, 1) else {
        return None;
    };

    results.first().map(|elem| elem.clone())
}
