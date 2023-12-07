use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReminderError {
    #[error(transparent)]
    DBOperationError(surrealdb::Error),

    #[error("Task(id: {}) is not found", .id)]
    TaskNotFound { id: String },

    #[error("Task(id: {}) is not found", .id)]
    UserNotFound { id: String },

    #[error("Task(id: {}) is not found", .id)]
    GroupNotFound { id: String },

    #[error("Failed to convert task record to task; Reason: {}", .cause)]
    RecordToTaskFailed { cause: String },

    #[error("Failed to convert user id to user identifier; Reason: {}", .cause)]
    UserIdToUserIdentifierFailed { cause: String },
}
