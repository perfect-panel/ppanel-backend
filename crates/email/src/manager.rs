use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

use crate::sender::Sender;
use crate::worker::{TaskInfo, Worker};

#[async_trait::async_trait]
pub trait TaskRepo: Send + Sync {
    async fn find_one(&self, id: i64) -> Result<TaskInfo, anyhow::Error>;
    async fn update(&self, data: &TaskInfo) -> Result<(), anyhow::Error>;
    async fn update_status(&self, id: i64, status: i16) -> Result<(), anyhow::Error>;
    fn is_cancelled(&self, id: i64) -> bool;
}

pub struct WorkerManager {
    repo: Arc<dyn TaskRepo>,
    sender: Arc<dyn Sender>,
    workers: RwLock<HashMap<i64, WorkerHandle>>,
}

struct WorkerHandle {
    worker: Arc<Worker>,
}

impl WorkerManager {
    pub fn new(repo: Arc<dyn TaskRepo>, sender: Arc<dyn Sender>) -> Arc<Self> {
        let manager = Arc::new(WorkerManager {
            repo,
            sender,
            workers: RwLock::new(HashMap::new()),
        });

        let mgr = manager.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(60)).await;
                mgr.check_workers().await;
            }
        });

        manager
    }

    pub async fn add_worker(&self, id: i64) {
        let mut workers = self.workers.write().await;
        if workers.contains_key(&id) {
            tracing::info!(
                "Batch Send Email: Worker already exists, task_id={}",
                id
            );
            return;
        }

        let worker = Arc::new(Worker::new(id, self.repo.clone(), self.sender.clone()));
        let handle = WorkerHandle {
            worker: worker.clone(),
        };
        workers.insert(id, handle);

        tracing::info!(
            "Batch Send Email: Added new worker, task_id={}",
            id
        );

        tokio::spawn(async move {
            worker.start().await;
        });
    }

    pub async fn get_worker(&self, id: i64) -> Option<Arc<Worker>> {
        let workers = self.workers.read().await;
        workers.get(&id).map(|h| h.worker.clone())
    }

    pub async fn remove_worker(&self, id: i64) {
        let mut workers = self.workers.write().await;
        if workers.remove(&id).is_some() {
            tracing::info!(
                "Batch Send Email: Removed worker, task_id={}",
                id
            );
        } else {
            tracing::error!(
                "Batch Send Email: Worker not found for removal, task_id={}",
                id
            );
        }
    }

    async fn check_workers(&self) {
        let mut workers = self.workers.write().await;
        let mut to_remove = Vec::new();

        for (&id, handle) in workers.iter() {
            if handle.worker.is_running().await as i16 == 2 {
                to_remove.push(id);
            }
        }

        for id in to_remove {
            workers.remove(&id);
            tracing::info!(
                "Batch Send Email: Removed completed worker, task_id={}",
                id
            );
        }
    }
}

static MANAGER: std::sync::OnceLock<Arc<WorkerManager>> = std::sync::OnceLock::new();

pub fn set_global_manager(manager: Arc<WorkerManager>) -> Result<(), Arc<WorkerManager>> {
    MANAGER.set(manager)
}

pub fn get_global_manager() -> Option<&'static Arc<WorkerManager>> {
    MANAGER.get()
}
