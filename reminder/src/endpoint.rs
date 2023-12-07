use tonic::Status;

use crate::log;

pub(crate) mod notification;
pub(crate) mod task;
pub(crate) mod user;

pub(self) fn invalid_argument_error<T>(msg: impl Into<String> + Copy) -> Result<T, Status> {
    log!("ERROR" -> format!("Invalid gRPC request arguments").bold().red());
    log!("ERROR" -> format!("Reason: {}", msg.into()).bold().red());
    log!("gRPC" -> ">>> Error InvalidArgument".cyan());

    Err::<T, Status>(Status::invalid_argument(msg))
}
