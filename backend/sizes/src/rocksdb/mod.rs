pub mod property;

use self::property::Property;
use crate::kvstore::KvStore;
use crate::rocksdb::property::PropertyPrefix;
use rocksdb::{BlockBasedOptions, ColumnFamilyDescriptor, DBCompactionStyle, DBPinnableSlice, LogLevel, Options, DB};
use std::path::{Path, PathBuf};

const DEFAULT_COLUMN_FAMILY_NAME: &str = "default";

const DEFAULT_BLOOM_FILTER_BIT: f64 = 10f64;

const TINY_WRITE_BUFFER_SIZE: usize = DEFAULT_WRITE_BUFFER_SIZE >> 2;
const TINY_BLOCK_CACHE_SIZE: usize = DEFAULT_BLOCK_CACHE_SIZE >> 2;

const DEFAULT_WRITE_BUFFER_SIZE: usize = 32 << 20;
const DEFAULT_BLOCK_CACHE_SIZE: usize = 64 << 20;

const HUGE_WRITE_BUFFER_SIZE: usize = DEFAULT_WRITE_BUFFER_SIZE << 2;
const HUGE_BLOCK_CACHE_SIZE: usize = DEFAULT_BLOCK_CACHE_SIZE << 2;

pub struct ColumnFamilyConfig {
    pub name: String,
    pub write_buffer_size: usize,
    pub block_cache_size: usize,
    pub bloom_filter_bit: f64
}

impl Into<ColumnFamilyDescriptor> for &ColumnFamilyConfig {
    fn into(self) -> ColumnFamilyDescriptor {
        let mut opt = Options::default();

        set_default_options(&mut opt);

        opt.optimize_level_style_compaction(self.write_buffer_size << 2);

        let mut bbto = BlockBasedOptions::default();
        bbto.set_bloom_filter(self.bloom_filter_bit, true);
        bbto.set_block_cache(&rocksdb::Cache::new_lru_cache(self.block_cache_size));
        // use 64K block size
        bbto.set_block_size(64 << 10);

        opt.set_write_buffer_size(self.write_buffer_size);
        opt.set_block_based_table_factory(&bbto);
        opt.set_compaction_style(DBCompactionStyle::Level);

        // use maximum 1024M for L1
        opt.set_max_bytes_for_level_base(1024 << 20);
        opt.set_max_bytes_for_level_multiplier(10f64);

        // use 512M for L1
        opt.set_target_file_size_base(512 << 20);
        opt.set_target_file_size_multiplier(10);

        ColumnFamilyDescriptor::new(self.name.clone(), opt)
    }
}

impl ColumnFamilyConfig {
    pub fn new(
        name: &str,
        write_buffer_size: usize,
        block_cache_size: usize,
        bloom_filter_bit: f64
    ) -> Self {
        ColumnFamilyConfig {
            name: name.to_string(),
            write_buffer_size,
            block_cache_size,
            bloom_filter_bit
        }
    }

    pub fn new_default(name: &str) -> Self {
        Self::new(name, DEFAULT_WRITE_BUFFER_SIZE, DEFAULT_BLOCK_CACHE_SIZE, DEFAULT_BLOOM_FILTER_BIT)
    }

    pub fn new_tiny(name: &str) -> Self {
        Self::new(name, TINY_WRITE_BUFFER_SIZE, TINY_BLOCK_CACHE_SIZE, DEFAULT_BLOOM_FILTER_BIT)
    }

    pub fn new_huge(name: &str) -> Self {
        Self::new(name, HUGE_WRITE_BUFFER_SIZE, HUGE_BLOCK_CACHE_SIZE, DEFAULT_BLOOM_FILTER_BIT)
    }
}

pub enum StdColumnFamilyConfig {
    TINY,
    DEFAULT,
    HUGE,
}

pub fn new_db_option() -> Options {
    let mut opt = Options::default();
    set_default_options(&mut opt);
    opt.create_if_missing(true);
    opt.create_missing_column_families(true);
    opt
}

fn set_default_options(opt: &mut Options) {
    opt.set_keep_log_file_num(1);
    opt.set_log_level(LogLevel::Warn);
    opt.set_max_log_file_size(128 << 20);
    opt.set_max_total_wal_size(128 << 20);
    opt.set_wal_ttl_seconds(60);
    opt.set_wal_size_limit_mb(8);
    // please refer to https://github.com/facebook/rocksdb/wiki/Rate-Limiter
    opt.set_ratelimiter(10<<20, 100000, 10);
}

pub fn is_valid_db(path: &Path) -> bool {
    let res = DB::open_for_read_only(&Options::default(), path, true);
    if res.is_err() {
        return false;
    }
    true
}

pub fn list_cfs(path: &Path) -> Vec<String> {
    DB::list_cf(&new_db_option(), path).unwrap_or(Vec::new())
}

pub struct RocksDBBuilder {
    path: PathBuf,
    cfs: Vec<ColumnFamilyConfig>,
    truncate: bool
}

pub struct RocksDB {
    pub(crate) db: DB
}

impl KvStore for RocksDB {
    fn get_bytes(&self, table: &str, key: impl AsRef<str>) -> Option<impl AsRef<[u8]>> {
        self.get_cf(table, key)
    }

    fn set<V>(&self, table: &str, key: impl AsRef<str>, value: V) -> crate::Result<()>
    where V: AsRef<[u8]> {
        self.put_cf(table, key, value)
    }

    fn foreach<F>(self: &Self, table: &str, key_prefix: impl AsRef<str>, limit: u32, callback: F)
    where F: FnMut(&str, &str) {
        self.prefix_foreach_cf(table, key_prefix, limit, callback)
    }

}

impl RocksDBBuilder {
    pub fn new(path: impl AsRef<str>) -> RocksDBBuilder {
        let mut builder = RocksDBBuilder {
            path: PathBuf::from(path.as_ref()),
            cfs: Vec::new(),
            truncate: false
        };
        builder.with_column_family(DEFAULT_COLUMN_FAMILY_NAME, StdColumnFamilyConfig::DEFAULT);
        builder
    }

    pub fn with_column_family(&mut self, name: &str, config: StdColumnFamilyConfig) -> &mut RocksDBBuilder {
        match config {
            StdColumnFamilyConfig::TINY => self.cfs.push(ColumnFamilyConfig::new_tiny(name)),
            StdColumnFamilyConfig::DEFAULT => self.cfs.push(ColumnFamilyConfig::new_default(name)),
            StdColumnFamilyConfig::HUGE => self.cfs.push(ColumnFamilyConfig::new_huge(name))
        };
        self
    }

    pub fn truncate(&mut self, val: bool) -> &mut RocksDBBuilder {
        self.truncate = val;
        self
    }

    pub fn build(self: &Self) -> RocksDB {
        let db_opts = new_db_option();
        let cf_descriptors: Vec<ColumnFamilyDescriptor> = self.cfs.iter().map(|x| x.into()).collect();
        if self.truncate && is_valid_db(self.path.as_path()) {
            DB::destroy(&db_opts, self.path.as_path()).unwrap();
        }

        let db = DB::open_cf_descriptors(&db_opts, self.path.as_path(), cf_descriptors).unwrap();
        RocksDB {
            db
        }
    }
}

impl RocksDB {
    pub fn get_property(self: &Self, prop: Property) -> String {
        self.db.property_value(format!("rocksdb.{}", prop).as_str())
            .map_or(String::new(), |x| x.unwrap_or(String::new()))
    }

    pub fn get_property_with_prefix(self: &Self, prop: PropertyPrefix, prefix: i32) -> String {
        let p = format!("{}{}", prop, prefix);
        self.get_property(p.as_str())
    }

    pub fn prefix_foreach_cf<F>(self: &Self, cf: &str, key_prefix: impl AsRef<str>, limit: u32, mut callback: F)
    where F: FnMut(&str, &str)  {
        let prefix = key_prefix.as_ref();
        let iter = self.db.prefix_iterator_cf(
            self.db.cf_handle(cf.as_ref()).unwrap(),
            prefix.as_bytes());
        let mut count = 0;
        for item in iter {
            if item.is_err() {
                eprintln!("prefix_foreach error: {}", item.unwrap_err());
                continue;
            }
            let (k, v) = item.unwrap();
            if !prefix.is_empty() && !k.starts_with(prefix.as_bytes()) {
                break;
            }
            unsafe {
                callback(
                    std::str::from_utf8_unchecked(&k),
                    std::str::from_utf8_unchecked(&v));
            }
            count += 1;
            if limit > 0 && limit == count {
                break;
            }
        }
    }

    fn get_cf (self: &Self, cf: &str, key: impl AsRef<str>) -> Option<DBPinnableSlice> {
        self.db.get_pinned_cf(
            self.db.cf_handle(cf).expect(format!("no column family handle for {}", cf).as_str()),
            key.as_ref()).unwrap_or(None)
    }

    fn put_cf<V>(self: &Self, cf: &str, key: impl AsRef<str>, value: V) -> crate::Result<()>
    where V: AsRef<[u8]> {
        match self.db.put_cf(self.db.cf_handle(cf).unwrap(), key.as_ref(), value) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into())
        }
    }
}