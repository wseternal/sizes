use crate::rocksdb::{RocksDB, RocksDBBuilder, StdColumnFamilyConfig};
use std::path::Path;
use std::sync::OnceLock;

pub mod dirstat;
pub mod scanresult;

pub static TABLE_CONF: &str = "confs";
pub static TABLE_DIR_STAT: &str = "dirs";
pub static TABLE_DIR_SCAN_RESULT: &str = "dirscanres";

static DB: OnceLock<RocksDB> = OnceLock::new();
pub fn get_db(path: &Path, truncate: bool) -> &'static RocksDB {
    DB.get_or_init(|| {
        let db = RocksDBBuilder::new(path.to_string_lossy())
            .with_column_family(TABLE_CONF, StdColumnFamilyConfig::TINY)
            .with_column_family(TABLE_DIR_STAT, StdColumnFamilyConfig::HUGE)
            .with_column_family(TABLE_DIR_SCAN_RESULT, StdColumnFamilyConfig::DEFAULT)
            .truncate(truncate)
            .build();
       db
    })
}
