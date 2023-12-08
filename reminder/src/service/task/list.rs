use anyhow::Result;

use crate::{
    domain::{
        task::{Task, TaskRepository},
        user::{UserIdentifier, UserRepository},
    },
    misc::error::ReminderError,
    service::service::TaskService,
};

impl<T: TaskRepository, U: UserRepository> TaskService<T, U> {
    pub(crate) async fn list_task(
        &self,
        who: Option<UserIdentifier>,
    ) -> Result<Vec<Task>, ReminderError> {
        let list_result = self.task_repo.list(who, None).await;

        list_result
    }
}
