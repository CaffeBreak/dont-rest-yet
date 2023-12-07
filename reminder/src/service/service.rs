use anyhow::Result;
use tokio::sync::Mutex;

use crate::{
    domain::{
        task::{Task, TaskRepository},
        user::UserRepository,
    },
    misc::id::Id,
};

pub(crate) struct TaskService<T>
where
    T: TaskRepository,
{
    pub(crate) task_repo: T,
}

pub(crate) struct UserService<T>
where
    T: UserRepository,
{
    pub(crate) user_repo: T,
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

    pub(crate) async fn add_cache(&self, task: Task) -> Result<()> {
        let mut locked_cache = self.task_cache.lock().await;
        locked_cache.push(task);

        Ok(())
    }

    pub(crate) async fn delete_cache(&self, id: Id) -> Result<()> {
        let mut locked_cache = self.task_cache.lock().await;
        locked_cache.retain(|task| task.id != id);

        Ok(())
    }

    pub(crate) async fn update_cache(&self, task: Task) -> Result<()> {
        let mut updated = false;
        let mut locked_cache = self.task_cache.lock().await;
        *locked_cache = locked_cache
            .iter()
            .map(|cache| {
                if cache.id == task.id {
                    updated = true;
                    task.clone()
                } else {
                    (*cache).clone()
                }
            })
            .collect();
        if !updated {
            locked_cache.push(task);
        }

        Ok(())
    }
}
