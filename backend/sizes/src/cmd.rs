use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::iter::Map;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use kanal::{AsyncReceiver, AsyncSender, SendError};
use tokio::sync::RwLock;

use crate::db::scanresult;
use crate::scan::DirScanResult;
use crate::{db, scandir, StaticBox};
use crate::conf::app_db_path;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Command {
    ScanDir(PathBuf),
}

pub enum ProgressStatus {
    PENDING,
    STARTED,
    COMPLETED,
    FAILED,
    ABORTED,
}

pub trait Progress: Display {
    fn new() -> Self;
    fn status() -> Result<ProgressStatus, String>;
    fn progress() -> Result<(i8, i8), String>;
}

type TASKS = Arc<RwLock<HashMap<Command, &'static DirScanResult>>>;

#[derive(Clone, Debug)]
pub struct TaskManager {
    ongoing_tasks: TASKS,
    tx: AsyncSender<Command>,
}

impl TaskManager {
    pub fn new() -> Self {
        let (tx, rx) = kanal::bounded_async(8);
        let task_manager = TaskManager {
            ongoing_tasks: TASKS::new(RwLock::new(HashMap::new())),
            tx,
        };

        let ret = task_manager.clone();
        tokio::spawn(async move {
            task_manager.command_manager_task(rx).await.unwrap();
        });
        ret
    }

    pub async fn send(&self, cmd: Command) -> Result<(), SendError> {
        self.tx.send(cmd).await
    }

    pub async fn scan_progress(&self) -> HashMap<PathBuf, DirScanResult> {
        let cmd = Command::ScanDir(PathBuf::from("/"));

        let mut tasks = HashMap::new();
        let r = self.ongoing_tasks.read().await;
        r.iter().for_each(|(k_ref, v_ref)| {
            if let Command::ScanDir(path) = k_ref {
                let mut elem = (*v_ref).clone();
                elem.ongoing = true;
                tasks.insert(path.clone(), elem);
            }
        });
        drop(r);

        tasks
    }

    async fn command_manager_task(&self, rx: AsyncReceiver<Command>) -> Result<(), Box<dyn Error>> {
        loop {
            let item = rx.recv().await;
            if item.is_err() {
                let err = item.err().unwrap();
                eprintln!("{}, exiting the manager task...", err);
                return Err(err.into());
            }
            self.process_command(item.unwrap()).await;
        }
    }

    async fn process_command(&self, cmd: Command) -> () {
        println!("received command {:?}", cmd);

        let r = self.ongoing_tasks.read().await;
        if r.contains_key(&cmd) {
            eprintln!("There is already a running task {:?}", cmd);
            return;
        }
        drop(r);

        let t1 = self.ongoing_tasks.clone();

        tokio::spawn(async move {
            let progress = StaticBox::new(DirScanResult::new());

            let mut w = t1.write().await;
            w.insert(cmd.clone(), progress.get());
            drop(w);
            println!("insert cmd into ongoing task queue");

            match cmd {
                Command::ScanDir(ref path) => {
                    scan_dir(&path, progress.get_mut()).await;
                }
            };

            let mut w = t1.write().await;
            w.remove(&cmd);
            drop(w);

            progress.drop();

            println!("finished cmd {:?} remove from ongoing task queue", cmd);
        });
    }
}

async fn scan_dir(path: &PathBuf, progress: &'static mut DirScanResult) {
    println!("start scanning directory {:?}", path);

    let db = db::get_db(app_db_path(), false);

    let t1 = Instant::now();
    scandir::compute_dir_stats_loop_parallel(db, path, progress)
        .await
        .unwrap();
    let elapsed = t1.elapsed().as_secs();
    progress.spent = elapsed;
    progress.ongoing = false;

    if let Err(e) = scanresult::save_dir_scan_result(db, path, progress) {
        eprintln!("save dir scan result failed: {e}");
    }
    println!(
        "spent {} seconds, finished scanning directory {}, stat: {:?}",
        elapsed,
        path.display(),
        progress
    );
}
