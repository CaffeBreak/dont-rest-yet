use tokio::sync::Mutex;

use crate::domain::{
    task::{Task, TaskRepository},
    user::UserRepository,
};

pub(crate) struct TaskService<T, U>
where
    T: TaskRepository,
    U: UserRepository,
{
    pub(crate) task_repo: T,
    pub(crate) user_repo: U,
}

pub(crate) struct UserService<T>
where
    T: UserRepository,
{
    pub(crate) user_repo: T,
}

pub(crate) struct NotificationService<T>
where
    T: TaskRepository,
{
    pub(crate) task_repo: T,
    pub(crate) task_cache: Mutex<Vec<Task>>,
}
