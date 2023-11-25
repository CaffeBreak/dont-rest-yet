use anyhow::Result;
use chrono::{DateTime, Utc};
use prost_types::Timestamp;

use crate::{driver::grpc_api, misc::id::Id};

use super::user::User;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Id,
    pub title: String,
    pub remind_at: DateTime<Utc>,
    pub who: User,
}
impl From<grpc_api::reminder::Task> for Task {
    fn from(value: grpc_api::reminder::Task) -> Self {
        let remind_at = value.remind_at.unwrap();
        Self {
            id: Id::from(value.id),
            title: value.title,
            remind_at: DateTime::<Utc>::from_timestamp(
                remind_at.seconds,
                remind_at.nanos.try_into().unwrap(),
            )
            .unwrap(),
            who: User { id: value.who },
        }
    }
}
impl Into<grpc_api::reminder::Task> for Task {
    fn into(self) -> grpc_api::reminder::Task {
        let seconds = self.remind_at.timestamp();
        let nanos = self.remind_at.timestamp_subsec_nanos().try_into().unwrap();

        grpc_api::reminder::Task {
            id: self.id.to_string(),
            title: self.title,
            remind_at: Some(Timestamp {
                seconds: seconds,
                nanos: nanos,
            }),
            who: self.who.id,
        }
    }
}

pub trait TaskRepository {
    fn create(
        &self,
        id: Id,
        title: String,
        remind_at: DateTime<Utc>,
        who: User,
    ) -> impl std::future::Future<Output = Result<Task>> + Send;
    fn list(
        &self,
        who: Option<User>,
    ) -> impl std::future::Future<Output = Result<Vec<Task>>> + Send;
    fn delete(&self, id: Id) -> impl std::future::Future<Output = Result<Task>> + Send;
}
