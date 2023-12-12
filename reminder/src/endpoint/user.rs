use tonic::{Request, Response, Status};

use crate::{
    driver::grpc_api::reminder::{
        user_service_server::UserService, DeleteUserRequest, ListUserRequest, User, UserIdentifier,
        Users,
    },
    endpoint::invalid_argument_error,
    init::USER_SERVICE,
    log,
};

pub(crate) struct UserSrv;

#[tonic::async_trait]
impl UserService for UserSrv {
    async fn create_user(
        &self,
        request: Request<UserIdentifier>,
    ) -> Result<Response<User>, Status> {
        let request = request.into_inner();
        log!("gRPC" -> format!("<<< Create user request received.").cyan());

        // 各パラメータのパース
        if request.client.is_empty() {
            return invalid_argument_error("Client is not found.");
        };
        if request.identifier.is_empty() {
            return invalid_argument_error("Identifier is not found.");
        };

        let create_result = USER_SERVICE.create_user(request.into()).await;

        match create_result {
            Ok(created) => {
                log!("gRPC" -> format!(">>> User created.").cyan());
                log!("DEBUG" -> format!("Created: {:?}", created).dimmed());

                Ok(Response::new(created.into()))
            }
            Err(error) => {
                log!("ERROR" -> format!("Create user falied").bold().red());
                log!("ERROR" -> format!("Reason: {}", error.to_string()).bold().red());

                Err(Status::internal(error.to_string()))
            }
        }
    }
    async fn list_user(
        &self,
        request: Request<ListUserRequest>,
    ) -> Result<Response<Users>, Status> {
        todo!()
    }
    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<User>, Status> {
        todo!()
    }
}
