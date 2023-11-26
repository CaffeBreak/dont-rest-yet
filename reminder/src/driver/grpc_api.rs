use std::net::SocketAddr;

use tonic::transport::Server;

use crate::{
    driver::grpc_api::reminder::FILE_DESCRIPTOR_SET,
    endpoint::{notification::NotificationSrv, task::TaskSrv},
    init::CONFIG,
    log,
};

use self::reminder::{
    notification_service_server::NotificationServiceServer, task_service_server::TaskServiceServer,
};

pub mod reminder {
    tonic::include_proto!("dry.reminder");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("reminder_descriptor");
}

pub async fn serve() -> anyhow::Result<()> {
    let addr: SocketAddr = format!("0.0.0.0:{}", CONFIG.grpc_port).parse().unwrap();
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    log!("gRPC" -> format!("Start listening at {}", addr.to_string()).cyan());
    Server::builder()
        .add_service(reflection_service)
        .add_service(TaskServiceServer::new(TaskSrv))
        .add_service(NotificationServiceServer::new(NotificationSrv))
        .serve(addr)
        .await?;

    Ok(())
}
