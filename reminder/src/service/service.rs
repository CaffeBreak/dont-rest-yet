use crate::domain::task::TaskRepository;

pub struct TaskService<T>
where
    T: TaskRepository,
{
    pub task_repo: T,
}
