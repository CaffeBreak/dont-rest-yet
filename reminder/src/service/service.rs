use anyhow::Result;
use tokio::sync::Mutex;

use crate::{
    domain::task::{Task, TaskRepository},
    misc::id::Id,
};

pub(crate) struct TaskService<T>
where
    T: TaskRepository,
{
    pub(crate) task_repo: T,
}

pub(crate) struct NotificationService<T>
where
    T: TaskRepository,
{
    pub(crate) task_repo: T,
    pub(crate) task_cache: Mutex<Vec<Task>>,
}
impl<T: TaskRepository> NotificationService<T> {
    pub(crate) fn new(task_repo: T) -> Self {
        Self {
            task_repo,
            task_cache: Mutex::new(vec![]),
        }
    }

    pub(crate) async fn sort_cache(&self) -> Result<()> {
        let mut locked_cache = self.task_cache.lock().await;
        locked_cache.sort_by(|a, b| a.id.clone().parse().cmp(&b.id.clone().parse()));

        Ok(())
    }

    pub(crate) async fn add_cache(&self, task: Task) -> Result<()> {
        {
            let mut locked_cache = self.task_cache.lock().await;
            locked_cache.push(task);
        }

        self.sort_cache().await?;

        Ok(())
    }

    pub(crate) async fn delete_cache(&self, id: Id) -> Result<()> {
        let target = self.task_repo.get(id).await?;

        {
            let mut locked_cache = self.task_cache.lock().await;
            locked_cache.retain(|task| task.id != target.id);
        }

        self.sort_cache().await?;

        Ok(())
    }
}
