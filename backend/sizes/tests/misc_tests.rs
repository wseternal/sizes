use sizes::Error;
use std::fs;

#[test]
fn test_global_conf_dir() {
    let dir = sizes::conf::app_db_path();
    println!("dir is {}", dir.display());
}

#[test]
fn read_file() {
    let content = fs::read_to_string("/Users/jiangzhaohua/tmp/a.txt").expect("open file failed");
    println!("content is {content}")
}

#[test]
fn test_common_error() {
    let s =  "some error description";
    let err = Error::new(s);
    println!("err is {}", err);
}