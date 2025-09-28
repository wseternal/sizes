use crate::cmd::TaskManager;
use crate::rocksdb::RocksDB;
use std::cell::UnsafeCell;
use std::env::{self};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use rocket::form::validate::msg;
use crate::conf::app_db_path;

pub mod cmd;
pub mod conf;
pub mod db;
pub mod scandir;
pub mod rocksdb;
pub mod kvstore;
pub mod scan;
pub mod task;

pub struct Client {
    pub task_manager: TaskManager,
    pub db: &'static RocksDB
}

pub async fn init() -> Client {
    let task_manager = TaskManager::new();
    let db = db::get_db(app_db_path(), false);
    Client { task_manager, db }
}

#[inline(always)]
pub fn unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn home_dir() -> PathBuf {
    let is_windows = env::var("OS")
        .map(|os| os.contains("windows"))
        .unwrap_or(false);
    if is_windows {
        env::var("LOCALAPPDATA").map(|v| PathBuf::from(v)).unwrap()
    } else {
        env::home_dir().unwrap()
    }
}

#[derive(Debug)]
pub struct StaticBox<T: ?Sized> {
    value: *mut T,
}

unsafe impl<T> Send for StaticBox<T> {}

impl<T> StaticBox<T> {
    pub fn new(value: T) -> StaticBox<T> {
        StaticBox {
            value: Box::leak(Box::new(UnsafeCell::new(value))).get(),
        }
    }

    // Pay attention the static lifetime here, you must ensure that
    // your code make the StaticBox itself live long enough, at least,
    // longer than all the mut/immut references
    pub fn get_mut(&self) -> &'static mut T {
        unsafe { &mut *self.value }
    }

    // Pay attention the static lifetime here, you must ensure that
    // your code make the StaticBox itself live long enough, at least,
    // longer than all the mut/immut references
    pub fn get(&self) -> &'static T {
        unsafe { &*self.value }
    }

    pub fn drop(&self) {
        unsafe {
            let _box = Box::from_raw(self.value);
        }
    }

    // you need guarantee that the `v` is got from `StaticBox::get` or `StaticBox::get_mut`
    pub fn drop_raw(v: &T) {
        unsafe {
            let p = v as *const T as *mut T;
            let _box = Box::from_raw(p);
        }
    }
}

#[derive(Debug)]
pub struct Error(String);

impl Error {
    pub fn new(msg: impl AsRef<str>) -> Error {
        Error(msg.as_ref().to_string())
    }
}

impl AsRef<str> for Error {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "message {:?}", self.0.as_str())
    }
}

type Result<T> = std::result::Result<T, Error>;

impl From<::rocksdb::Error> for Error {
    fn from(value: ::rocksdb::Error) -> Self {
        Self(value.into_string())
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self(value)
    }
}