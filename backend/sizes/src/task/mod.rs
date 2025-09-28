use rocket::yansi::Paint;
use std::collections::VecDeque;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::{JoinHandle, JoinSet};

pub struct AsyncRecursiveTask<T, R> where T: Sized, R: Sized {
    pendings: Arc<RwLock<VecDeque<T>>>,
    max_worker_count: u32,
    one_task: fn (&mut Self, T) -> R,
    workers: JoinSet<R>
}

pub struct AsyncRecursiveTaskBuilder<T, R> where T: Sized, R: Sized {
    max_worker_count: u32,
    one_task: fn (&mut AsyncRecursiveTask<T,R>, T) -> R
}

impl<T, R> AsyncRecursiveTaskBuilder<T, R> where T: Sized, R: Sized {
    pub fn new() -> AsyncRecursiveTaskBuilder<T, R> {
        AsyncRecursiveTaskBuilder {
            one_task: ,
            max_worker_count: 0,
        }
    }
}

impl<T, R> AsyncRecursiveTask<T, R> where T: Sized , R: Sized {
    pub async fn pending_count(&self) -> usize {
        let r = self.pendings.read().await;
        let count = r.len();
        drop(r);
        count
    }

    pub async fn add_tasks(&self, mut tasks: Vec<T>) {
        let mut w = self.pendings.write().await;
        tasks.into_iter().for_each(|t| w.push_back(t));
        drop(w);
    }

    pub async fn add_task(&self, task: T) {
        let mut w = self.pendings.write().await;
        w.push_back(task);
        drop(w);
    }

    pub async fn start_with(&mut self, first_task: T) {
        self.add_task(first_task).await;
        self.start()
    }

    pub async fn start(&mut self) -> JoinHandle<()> {
        tokio::spawn(async move {
            let num_threads = std::thread::available_parallelism()
                .unwrap_or(NonZeroUsize::new(8).unwrap())
                .get();

            loop {
                let pending_task_count = self.pending_count().await;
                if pending_task_count == 0 && self.workers.is_empty() {
                    break
                }
                // busy working or in finishing, wait one worker done
                if self.workers.len() > num_threads || pending_task_count == 0 {
                    let out: R = self.workers.join_next().await.unwrap().unwrap();

                    progress.scanned += &dir_overview;
                    if dir_overview.is_cached {
                        progress.cached += &dir_overview;
                    }
                }
            }
        })
    }
}
