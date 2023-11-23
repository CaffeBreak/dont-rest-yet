use crate::{
    domain::task::{Task, TaskRepository},
    misc::id::Id,
};
use anyhow::{Ok, Result};

use super::service::TaskService;

impl<T: TaskRepository> TaskService<T> {
    pub async fn delete_task(&self, id: Id) -> Result<Task> {
        let deleted = self.task_repo.delete(id).await?;

        Ok(deleted)
    }
}
