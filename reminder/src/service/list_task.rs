use crate::{
    domain::{
        task::{Task, TaskRepository},
        user::User,
    },
    misc::error::ReminderError,
};
use anyhow::Result;

use super::service::TaskService;

impl<T: TaskRepository> TaskService<T> {
    pub async fn list_task(&self, who: Option<User>) -> Result<Vec<Task>, ReminderError> {
        let list_result = self.task_repo.list(who, None).await;

        list_result
    }
}
