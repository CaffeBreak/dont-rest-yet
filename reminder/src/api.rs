use tonic::{transport::Server, Request, Response, Status};

use crate::misc::id::Id;

use self::reminder::{
    task_service_server::{TaskService, TaskServiceServer},
    CreateTaskRequest, DeleteTaskRequest, ListTaskRequest, Task, Tasks,
};

pub mod reminder {
    tonic::include_proto!("dry.reminder");
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
        println!("request: {:?}", create_task_request);

        Ok(Response::new(Task {
            id: Id::new().to_string(),
            title: create_task_request.title,
            remind_at: create_task_request.remind_at,
            who: create_task_request.who,
        }))
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
    let addr = "0.0.0.0:58946".parse().unwrap();

    Server::builder()
        .add_service(TaskServiceServer::new(TaskSrv))
        .serve(addr)
        .await?;

    anyhow::Ok(())
}
