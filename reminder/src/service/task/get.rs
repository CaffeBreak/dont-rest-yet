use anyhow::Result;

use crate::{
    domain::{
        task::{Task, TaskRepository},
        user::UserRepository,
    },
    misc::{error::ReminderError, id::Id},
    service::service::TaskService,
};

impl<T: TaskRepository, U: UserRepository> TaskService<T, U> {
    pub(crate) async fn get_task(&self, id: Id) -> Result<Task, ReminderError> {
        let get_result = self.task_repo.get(id).await;

        get_result
    }
}
