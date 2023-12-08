use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::{
    domain::{
        task::{Task, TaskRepository},
        user::UserRepository,
    },
    init::{CONFIG, NOTIFICATION_SERVICE},
    misc::{error::ReminderError, id::Id},
    service::service::TaskService,
};

impl<T: TaskRepository, U: UserRepository> TaskService<T, U> {
    pub(crate) async fn update_task(
        &self,
        id: Id,
        title: Option<String>,
        remind_at: Option<DateTime<Utc>>,
    ) -> Result<Task, ReminderError> {
        let target = self.task_repo.get(id.clone()).await?;
        let updated_result = self.task_repo.update(target.id, title, remind_at).await;

        if let Ok(task) = updated_result {
            let diff = task.remind_at - Utc::now();
            if diff.num_minutes() >= 0
                && diff.num_minutes() <= (CONFIG.notification_cache_interval * 3).into()
            {
                let cache_task = task.clone();
                tokio::spawn(async move {
                    NOTIFICATION_SERVICE.update_cache(cache_task).await.unwrap();
                });
            }

            Ok(task)
        } else {
            updated_result
        }
    }
}
