use std::{collections::VecDeque, io, os::macos::fs::MetadataExt, path::{Path, PathBuf}};

use common::get_tokio_runtime;
use sizes::scan::{DirScanOverview, DirScanResult};
use sizes::scandir::compute_dir_stats_recursive;
use sizes::db::get_db;

mod common;

#[test]
fn test_scandir_recurisve() {
    let runtime = get_tokio_runtime();

    runtime.block_on(async {
        let db = get_db(Path::new("/tmp/test.db"), false);

        let mut scan_result = DirScanResult::new();
        let res = compute_dir_stats_recursive(
            &db,
            Path::new("/Users/jiangzhaohua/tmp"),
            &mut scan_result
        ).await;
        println!("res is {:?}", scan_result);
    });
}

async fn compute_dir_stats_loop(path: &Path, overview: &mut DirScanOverview) -> io::Result<()> {
    let mut path_buf = PathBuf::new();
    path_buf.push(path);

    let mut pendings = VecDeque::from(vec![path_buf]);
    loop {
        if pendings.len() == 0 {
            println!("pendings are empty, exit loop");
            break;
        }

        let path = pendings.pop_front().unwrap();

        let Ok(entries) = path.read_dir() else {
            eprintln!("read_dir failed on path {:?}", path);
            continue;
        };

        for entry in entries {
            let Ok(entry) = entry else { continue };
            let Ok(meta) = entry.metadata() else { continue };

            if !(meta.is_dir() || meta.is_file()) {
                continue;
            }

            if meta.is_file() {
                overview.files += 1;
                overview.blocks += meta.st_blocks();
                continue;
            }
            overview.dirs += 1;

            let mut p = PathBuf::new();
            p.push(entry.path());

            pendings.push_back(p);
        }
    }
    Ok(())
}
