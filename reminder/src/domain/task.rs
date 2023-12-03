use std::future::Future;

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use prost_types::Timestamp;

use crate::{
    driver::grpc_api,
    misc::{error::ReminderError, id::Id},
};

use super::user::User;

#[derive(Debug, Clone)]
pub(crate) struct Task {
    pub(crate) id: Id,
    pub(crate) title: String,
    pub(crate) remind_at: DateTime<Utc>,
    pub(crate) who: User,
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
            remind_at: Some(Timestamp { seconds, nanos }),
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
    ) -> impl Future<Output = Result<Task, ReminderError>> + Send;
    fn get(&self, id: Id) -> impl Future<Output = Result<Task, ReminderError>> + Send;
    fn list(
        &self,
        who: Option<User>,
        duration: Option<Duration>,
    ) -> impl Future<Output = Result<Vec<Task>, ReminderError>> + Send;
    fn delete(&self, id: Id) -> impl Future<Output = Result<Task, ReminderError>> + Send;
    fn update(
        &self,
        id: Id,
        title: Option<String>,
        remind_at: Option<DateTime<Utc>>,
    ) -> impl Future<Output = Result<Task, ReminderError>> + Send;
}
