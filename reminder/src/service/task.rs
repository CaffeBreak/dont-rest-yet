use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::{
    domain::{
        task::{Task, TaskRepository},
        user::{UserIdentifier, UserRepository},
    },
    init::{CONFIG, NOTIFICATION_SERVICE},
    misc::{error::ReminderError, id::Id},
};

use super::service::TaskService;

impl<T: TaskRepository, U: UserRepository> TaskService<T, U> {
    pub(crate) async fn create_task(
        &self,
        title: String,
        remind_at: DateTime<Utc>,
        who: UserIdentifier,
    ) -> Result<Task, ReminderError> {
        let user_result = self.user_repo.get(who.clone()).await;
        let user = match user_result {
            Ok(user) => user.user_identifier,
            Err(error) => match error {
                ReminderError::UserNotFound { .. } => {
                    let user = self.user_repo.create(who).await?;

                    user.user_identifier
                }
                _ => return Err(error),
            },
        };

        let created_result = self
            .task_repo
            .create(Id::new(), title, remind_at, user)
            .await;

        if let Ok(task) = created_result {
            let diff = task.remind_at - Utc::now();
            if diff.num_minutes() >= 0
                && diff.num_minutes() <= (CONFIG.notification_cache_interval * 3).into()
            {
                let cache_task = task.clone();
                tokio::spawn(async move {
                    NOTIFICATION_SERVICE.add_cache(cache_task).await.unwrap();
                });
            }

            Ok(task)
        } else {
            created_result
        }
    }

    pub(crate) async fn list_task(
        &self,
        who: Option<UserIdentifier>,
    ) -> Result<Vec<Task>, ReminderError> {
        let list_result = self.task_repo.list(who, None).await;

        list_result
    }

    pub(crate) async fn get_task(&self, id: Id) -> Result<Task, ReminderError> {
        let get_result = self.task_repo.get(id).await;

        get_result
    }

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

    pub(crate) async fn delete_task(&self, id: Id) -> Result<Task, ReminderError> {
        let target = self.task_repo.get(id.clone()).await?;
        let delete_result = self.task_repo.delete(target.id).await;

        if let Ok(task) = delete_result {
            let diff = task.remind_at - Utc::now();
            if diff.num_minutes() >= 0
                && diff.num_minutes() <= (CONFIG.notification_cache_interval * 3).into()
            {
                let cache_task = task.clone();
                tokio::spawn(async move {
                    NOTIFICATION_SERVICE
                        .delete_cache(cache_task.id)
                        .await
                        .unwrap();
                });
            }

            Ok(task)
        } else {
            delete_result
        }
    }
}
