use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::{
    domain::{
        task::{Task, TaskRepository},
        user::User,
    },
    init::{CONFIG, NOTIFICATION_SERVICE},
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

        if let Ok(task) = created_result {
            let diff = task.remind_at - Utc::now();
            if diff.num_minutes() >= 0
                && diff.num_minutes() <= (CONFIG.notification_cache_interval * 3).into()
            {
                NOTIFICATION_SERVICE.add_cache(task.clone()).await.unwrap();
            }

            Ok(task)
        } else {
            created_result
        }
    }
}
