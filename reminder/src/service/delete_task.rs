use crate::{
    domain::task::{Task, TaskRepository},
    misc::{error::ReminderError, id::Id},
};
use anyhow::Result;

use super::service::TaskService;

impl<T: TaskRepository> TaskService<T> {
    pub async fn delete_task(&self, id: Id) -> Result<Task, ReminderError> {
        let target = self.task_repo.get(id.clone()).await?;

        Ok(self.task_repo.delete(target.id).await.unwrap())
    }
}
