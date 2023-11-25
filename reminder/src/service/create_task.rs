use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::{
    domain::{
        task::{Task, TaskRepository},
        user::User,
    },
    misc::{error::ReminderError, id::Id},
};

use super::service::TaskService;

impl<T: TaskRepository> TaskService<T> {
    pub async fn create_task(
        &self,
        title: String,
        remind_at: DateTime<Utc>,
        who: User,
    ) -> Result<Task, ReminderError> {
        let created_result = self
            .task_repo
            .create(Id::new(), title, remind_at, who)
            .await;

        created_result
    }
}
