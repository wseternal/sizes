use rocksdb::DEFAULT_COLUMN_FAMILY_NAME;
use sizes::kvstore::KvStore;
use sizes::rocksdb::{property, RocksDBBuilder};

#[cfg(test)]

#[test]
fn test_rocksdb_read_write() {
    let db = RocksDBBuilder::new("/tmp/test.db")
        .truncate(true)
        .build();

    let table = DEFAULT_COLUMN_FAMILY_NAME;
    db.set(table, "k1", "this is some value").unwrap();

    let res = db.get_string(table, "k1");

    println!("{:?}", res);
}

#[test]
fn test_prefix_read() {
    let db = &RocksDBBuilder::new("/tmp/test.db")
        .truncate(true)
        .build();
    let table = DEFAULT_COLUMN_FAMILY_NAME;


    db.set(table, "/foo", "foo").unwrap();
    db.set(table, "/foo/bar", "/foo/bar").unwrap();
    db.set(table, "/foo/zoo", "/foo/zoo").unwrap();
    db.set(table, "/bar", "/bar").unwrap();

    db.foreach(table, "/foo", 0, |k,v| { println!("{:?} -> {:?}", k, v); })
}

#[test]
fn test_rocksdb_sizes() {
    let db = RocksDBBuilder::new("/tmp/test.db")
        .build();
    println!("{}", db.get_property(property::KEstimateNumKeys));
}