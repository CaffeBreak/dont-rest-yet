use crate::domain::{
    task::{Task, TaskRepository},
    user::User,
};
use anyhow::{Ok, Result};

use super::service::TaskService;

impl<T: TaskRepository> TaskService<T> {
    pub async fn list_task(&self, who: Option<User>) -> Result<Vec<Task>> {
        let list = self.task_repo.list(who).await?;

        Ok(list)
    }
}
