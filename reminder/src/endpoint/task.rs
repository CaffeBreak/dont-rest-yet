use chrono::{DateTime, Utc};
use tonic::{Request, Response, Status};

use crate::{
    domain::{self, user::User},
    driver::grpc_api::reminder::{
        task_service_server::TaskService, CreateTaskRequest, DeleteTaskRequest, ListTaskRequest,
        Task, Tasks, UpdateTaskRequest,
    },
    init::TASK_SERVICE,
    log,
    misc::{error::ReminderError, id::Id},
};

#[derive(Debug)]
pub(crate) struct TaskSrv;

#[tonic::async_trait]
impl TaskService for TaskSrv {
    async fn create_task(
        &self,
        request: Request<CreateTaskRequest>,
    ) -> Result<Response<Task>, Status> {
        let request = request.into_inner();
        log!("gRPC" -> format!("<<< Create task request received.").cyan());

        // 各パラメータのパース
        let title = if !request.title.is_empty() {
            request.title
        } else {
            return invalid_argument_error("Title is not found.");
        };
        let who = if request.who.is_empty() {
            return invalid_argument_error("Who is not found.");
        } else {
            request.who
        };
        let remind_at = match request.remind_at {
            Some(remind_at) => remind_at,
            None => return invalid_argument_error("RemindAt is not found."),
        };

        let create_result = TASK_SERVICE
            .create_task(
                title,
                DateTime::<Utc>::from_timestamp(remind_at.seconds, remind_at.nanos as u32).unwrap(),
                User { id: who },
            )
            .await;

        match create_result {
            Ok(created) => {
                log!("gRPC" -> format!(">>> Task created.").cyan());
                log!("DEBUG" -> format!("Created: {:?}", created).dimmed());

                Ok(Response::new(created.into()))
            }
            Err(error) => {
                log!("ERROR" -> format!("Create task falied").bold().red());
                log!("ERROR" -> format!("Reason: {}", error.to_string()).bold().red());

                Err(Status::internal(error.to_string()))
            }
        }
    }
    async fn list_task(
        &self,
        request: Request<ListTaskRequest>,
    ) -> Result<Response<Tasks>, Status> {
        let request = request.into_inner();
        log!("gRPC" -> format!("<<< List task request received.").cyan());

        let list_result = TASK_SERVICE
            .list_task(match request.who {
                Some(who) => Some(User { id: who }),
                None => None,
            })
            .await;

        match list_result {
            Ok(list) => {
                log!("gRPC" -> format!(">>> Task listed.").cyan());
                log!("DEBUG" -> format!("Listed: {:?}", list).dimmed());
                Ok(Response::new(Tasks {
                    tasks: list
                        .iter()
                        .map(|task| domain::task::Task::into(task.clone()))
                        .collect(),
                }))
            }
            Err(error) => {
                log!("ERROR" -> format!("List task falied").bold().red());
                log!("ERROR" -> format!("Reason: {}", error.to_string()).bold().red());

                Err(Status::internal(error.to_string()))
            }
        }
    }
    async fn delete_task(
        &self,
        request: Request<DeleteTaskRequest>,
    ) -> Result<Response<Task>, Status> {
        let request = request.into_inner();
        log!("gRPC" -> format!("<<< Delete task request received.").cyan());

        // パラメータのパース
        let id = if !request.id.is_empty() {
            Id::from(request.id)
        } else {
            return invalid_argument_error("ID is not found.");
        };

        let deleted_result = TASK_SERVICE.delete_task(id).await;

        match deleted_result {
            Ok(deleted) => {
                log!("gRPC" -> format!(">>> Task deleted.").cyan());
                log!("DEBUG" -> format!("Deleted: {:?}", deleted).dimmed());

                Ok(Response::new(deleted.into()))
            }
            Err(error) => {
                log!("ERROR" -> format!("Delete task falied").bold().red());
                log!("ERROR" -> format!("Reason: {}", error.to_string()).bold().red());
                let error = match error {
                    ReminderError::DBOperationError(error) => Status::internal(error.to_string()),
                    ReminderError::TaskNotFound { id } => {
                        Status::not_found(format!("Task(id: {}) is not found.", id))
                    }
                };

                Err(error)
            }
        }
    }
    async fn update_task(
        &self,
        request: Request<UpdateTaskRequest>,
    ) -> Result<Response<Task>, Status> {
        let request = request.into_inner();
        log!("gRPC" -> "<<< Update task request received.".cyan());

        // パラメータのパース
        let id = if !request.id.is_empty() {
            Id::from(request.id)
        } else {
            return invalid_argument_error("ID is not found.");
        };

        let updated_result = TASK_SERVICE
            .update_task(
                id,
                request.title,
                match request.remind_at {
                    Some(remind_at) => {
                        DateTime::<Utc>::from_timestamp(remind_at.seconds, remind_at.nanos as u32)
                    }
                    None => None,
                },
            )
            .await;

        match updated_result {
            Ok(updated) => {
                log!("gRPC" -> format!(">>> Task updated.").cyan());
                log!("DEBUG" -> format!("Updated: {:?}", updated).dimmed());

                Ok(Response::new(updated.into()))
            }
            Err(error) => {
                log!("ERROR" -> format!("Update task falied").bold().red());
                log!("ERROR" -> format!("Reason: {}", error.to_string()).bold().red());
                let error = match error {
                    ReminderError::DBOperationError(error) => Status::internal(error.to_string()),
                    ReminderError::TaskNotFound { id } => {
                        Status::not_found(format!("Task(id: {}) is not found.", id))
                    }
                };

                Err(error)
            }
        }
    }
}

fn invalid_argument_error<T>(msg: impl Into<String> + std::marker::Copy) -> Result<T, Status> {
    log!("ERROR" -> format!("Invalid gRPC request arguments").bold().red());
    log!("ERROR" -> format!("Reason: {}", msg.into()).bold().red());
    log!("gRPC" -> ">>> Error InvalidArgument".cyan());

    Err::<T, Status>(Status::invalid_argument(msg))
}
