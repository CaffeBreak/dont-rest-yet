use std::pin::Pin;

use tokio::sync::{mpsc, oneshot};
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Response, Status};

use crate::{
    driver::grpc_api::reminder::{
        notification_service_server::NotificationService, PushNotificationRequest, Task,
        UserIdentifier,
    },
    init::NOTIFICATION_SERVICE,
    log,
    misc::stream::DropReceiver,
};

#[derive(Debug)]
pub(crate) struct NotificationSrv;

#[tonic::async_trait]
impl NotificationService for NotificationSrv {
    type PushNotificationStream =
        Pin<Box<dyn Stream<Item = Result<Task, Status>> + Send + 'static>>;

    async fn push_notification(
        &self,
        _: Request<PushNotificationRequest>,
    ) -> Result<Response<Self::PushNotificationStream>, Status> {
        log!("gRPC" -> format!("<<< Start push notification stream.").cyan());

        let (tx, rx) = mpsc::channel(256);
        let join_handle = tokio::spawn(async move {
            let mut stream = Box::pin(NOTIFICATION_SERVICE.send_notification());

            while let Some(task) = stream.next().await {
                let task_debug: Task = task.clone().into();
                let sent = tx.send(Ok(task.into())).await;
                match sent {
                    Ok(_) => {
                        log!("gRPC" -> format!(">>> A notification sent.").cyan());
                        log!("DEBUG" -> format!("Sent: {:?}", task_debug).dimmed());
                    }
                    Err(error) => {
                        log!("ERROR" -> "Failed to send a message.".bold().red());
                        log!("ERROR" -> format!("Reason: {}", error.to_string()).bold().red());

                        break;
                    }
                };
            }

            log!("gRPC" -> format!(">>> Close push notification stream.").cyan());
        });

        let (oneshot_tx, _) = oneshot::channel();
        let output_stream = DropReceiver {
            chan: Some(oneshot_tx),
            inner: rx,
            join_handle,
        };

        Ok(Response::new(Box::pin(output_stream)))
    }
}
