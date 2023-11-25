use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReminderError {
    #[error(transparent)]
    DBOperationError(surrealdb::Error),

    #[error("Task(id: {}) is not found", .id)]
    TaskNotFound { id: String },
}
