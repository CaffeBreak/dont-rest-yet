use crate::{
    domain::task::{Task, TaskRepository},
    init::{CONFIG, NOTIFICATION_SERVICE},
    misc::{error::ReminderError, id::Id},
};
use anyhow::Result;
use chrono::{Timelike, Utc};

use super::service::TaskService;

impl<T: TaskRepository> TaskService<T> {
    pub async fn delete_task(&self, id: Id) -> Result<Task, ReminderError> {
        let target = self.task_repo.get(id.clone()).await?;
        let delete_result = self.task_repo.delete(target.id).await;

        if let Ok(task) = delete_result {
            if task.remind_at.minute() as i32 - Utc::now().minute() as i32
                <= (CONFIG.notification_cache_interval * 3).into()
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
