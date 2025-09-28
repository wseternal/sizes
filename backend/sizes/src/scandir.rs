use crate::rocksdb::RocksDB;
use crate::scan;
use futures::future::BoxFuture;
use futures::FutureExt;
use std::collections::VecDeque;
use std::fmt::Display;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinSet;
use crate::kvstore::KvStore;
use crate::scan::{DirScanOverview, DirScanResult};

pub fn compute_dir_stats_recursive<'a>(
    db: &'a RocksDB,
    path: &'a Path,
    scan_result: &'a mut DirScanResult
) -> BoxFuture<'a, io::Result<()>> { async move {
    let (dir_stat, is_cached) = scan::scan_one_dir(db, path);
    let mut overview = DirScanOverview::new();
    overview.blocks = dir_stat.blocks;
    overview.files = dir_stat.file_num;
    overview.dirs = dir_stat.subdir_num;
    if is_cached {
        overview.is_cached = true;
        scan_result.cached += &overview;
    }
    scan_result.scanned += &overview;

    for elem in dir_stat.sub_dirs {
        let mut buf = PathBuf::from(path);
        buf.push(elem);
        compute_dir_stats_recursive(db, buf.as_path(), scan_result).await.unwrap()
    }
    Ok(())
}.boxed()}

async fn process_one_dir(
    db: &impl KvStore,
    path: PathBuf,
    todos: Arc<RwLock<VecDeque<PathBuf>>>,
) -> DirScanOverview {
    let (dir_stat, is_cached) = scan::scan_one_dir(db, path.as_path());
    let mut w1 = todos.write().await;
    for elem in dir_stat.sub_dirs {
        let mut buf = path.clone();
        buf.push(elem);
        w1.push_back(buf);
    }
    drop(w1);

    let mut overview = DirScanOverview::new();
    overview.blocks = dir_stat.blocks;
    overview.files = dir_stat.file_num;
    overview.dirs = dir_stat.subdir_num;
    if is_cached {
        overview.is_cached = true;
    }
    overview
}

pub async fn compute_dir_stats_loop_parallel(
    db: &'static RocksDB,
    root_path: &PathBuf,
    progress: &mut DirScanResult,
) -> io::Result<()> {
    let todos = Arc::new(RwLock::new(VecDeque::from(vec![root_path.clone()])));
    let mut jobs = JoinSet::new();

    loop {
        let r = todos.read().await;
        let left = r.len();
        drop(r);

        if jobs.is_empty() && left == 0 {
            break;
        }

        if jobs.len() > 8 || left == 0 {
            let dir_overview: DirScanOverview = jobs.join_next().await.unwrap().unwrap();
            progress.scanned += &dir_overview;
            if dir_overview.is_cached {
                progress.cached += &dir_overview;
            }
        }

        if left == 0 {
            continue;
        }

        let mut w = todos.write().await;
        let path = w.pop_front().unwrap();
        drop(w);

        jobs.spawn(process_one_dir(db, path, todos.clone()));
    }
    Ok(())
}
