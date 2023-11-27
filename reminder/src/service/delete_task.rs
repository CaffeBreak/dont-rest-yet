use crate::{
    domain::task::{Task, TaskRepository},
    init::{CONFIG, NOTIFICATION_SERVICE},
    misc::{error::ReminderError, id::Id},
};
use anyhow::Result;
use chrono::Utc;

use super::service::TaskService;

impl<T: TaskRepository> TaskService<T> {
    pub async fn delete_task(&self, id: Id) -> Result<Task, ReminderError> {
        let target = self.task_repo.get(id.clone()).await?;
        let delete_result = self.task_repo.delete(target.id).await;

        if let Ok(task) = delete_result {
            let diff = task.remind_at - Utc::now();
            if diff.num_minutes() >= 0
                && diff.num_minutes() <= (CONFIG.notification_cache_interval * 3).into()
            {
                NOTIFICATION_SERVICE
                    .delete_cache(task.clone().id)
                    .await
                    .unwrap();
            }

            Ok(task)
        } else {
            delete_result
        }
    }
}
