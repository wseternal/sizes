use sizes::conf::{add_watch, list_watch, WatchDirectoryConfiguration};
use sizes::kvstore::KvStore;
use sizes::rocksdb::{RocksDBBuilder, StdColumnFamilyConfig};
use std::collections::HashMap;
use sizes::db::TABLE_CONF;

mod common;

#[test]
fn test_db_get_set_str() {
    let db = RocksDBBuilder::new("/tmp/test.db")
        .truncate(true)
        .build();
    db.set("default", "k1", "hello").unwrap();
    let v = db.get_string("default", "k1");
    assert_eq!(v, "hello");
}

#[test]
fn test_db_get_set_watch() {
    let db = &RocksDBBuilder::new("/tmp/test.db")
        .with_column_family(TABLE_CONF, StdColumnFamilyConfig::TINY)
        .truncate(true)
        .build();

    let mut watches = list_watch(db);
    let empty: Vec<WatchDirectoryConfiguration> = vec![];
    assert!(watches == empty);

    let watch_conf_1 = WatchDirectoryConfiguration {
        refresh_interval: "1 Day".to_string(),
        label: "label1".to_string(),
        path: "path1".to_string(),
    };

    let watch_conf_2 = WatchDirectoryConfiguration {
        refresh_interval: "2 Day".to_string(),
        label: "label2".to_string(),
        path: "path2".to_owned(),
    };
    let mut watch_conf_3 = watch_conf_2.clone();
    watch_conf_3.label = "label3".to_string();
    watch_conf_3.refresh_interval = "monthly".to_string();

    add_watch(db, &watch_conf_1).unwrap();
    watches = list_watch(db);
    assert_eq!(watches, vec![watch_conf_1.clone()]);

    add_watch(db, &watch_conf_2).unwrap();
    watches = list_watch(db);
    assert_eq!(watches, vec![watch_conf_1.clone(), watch_conf_2]);

    // add watch_conf_3 that has the same path as conf 2, the list won't be changed
    add_watch(db, &watch_conf_3).unwrap();
    watches = list_watch(db);
    assert_eq!(watches, vec![watch_conf_1, watch_conf_3]);
}

#[test]
fn serialize_map() {
    let m = &vec![("k1".to_string(), 1), ("k2".to_string(), 2)]
        .into_iter()
        .collect::<HashMap<_, _>>();
    print!("m is {:?}", serde_json::to_value(m).unwrap());
}
