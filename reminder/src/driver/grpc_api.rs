use std::net::SocketAddr;

use chrono::{TimeZone, Utc};
use tonic::{transport::Server, Request, Response, Status};

use crate::{
    domain::user::User, driver::grpc_api::reminder::FILE_DESCRIPTOR_SET, init::TASK_SERVICE, log,
};

use self::reminder::{
    task_service_server::{TaskService, TaskServiceServer},
    CreateTaskRequest, DeleteTaskRequest, ListTaskRequest, Task, Tasks,
};

pub mod reminder {
    tonic::include_proto!("dry.reminder");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("reminder_descriptor");
}

#[derive(Debug)]
struct TaskSrv;

#[tonic::async_trait]
impl TaskService for TaskSrv {
    async fn create_task(
        &self,
        request: Request<CreateTaskRequest>,
    ) -> Result<Response<Task>, Status> {
        let create_task_request = request.into_inner();
        log!("INFO" | "Create task request received.");

        let remind_at = create_task_request.remind_at.unwrap();
        let remind_at_seconds = remind_at.seconds * 1_000_000_000;
        let remind_at_nanos = i64::from(remind_at.nanos);
        let created = TASK_SERVICE
            .create_task(
                create_task_request.title,
                Utc.timestamp_nanos(remind_at_seconds + remind_at_nanos),
                User {
                    id: create_task_request.who,
                },
            )
            .await
            .unwrap();

        Ok(Response::new(created.into()))
    }
    async fn list_task(
        &self,
        _request: Request<ListTaskRequest>,
    ) -> Result<Response<Tasks>, Status> {
        Ok(Response::new(Tasks { tasks: vec![] }))
    }
    async fn delete_task(
        &self,
        _request: Request<DeleteTaskRequest>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }
}

pub async fn serve() -> anyhow::Result<()> {
    let addr: SocketAddr = "0.0.0.0:58946".parse().unwrap();
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    log!("SERVER" -> format!("Start listening at {}", addr.to_string()).cyan());
    Server::builder()
        .add_service(reflection_service)
        .add_service(TaskServiceServer::new(TaskSrv))
        .serve(addr)
        .await?;

    anyhow::Ok(())
}
