use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use prost_types::Timestamp;

use crate::{
    driver::grpc_api,
    misc::{error::ReminderError, id::Id},
};

use super::user::UserIdentifier;

#[derive(Debug, Clone)]
pub(crate) struct Task {
    pub(crate) id: Id,
    pub(crate) title: String,
    pub(crate) remind_at: DateTime<Utc>,
    pub(crate) who: UserIdentifier,
}
impl Into<grpc_api::reminder::Task> for Task {
    fn into(self) -> grpc_api::reminder::Task {
        let seconds = self.remind_at.timestamp();
        let nanos = self.remind_at.timestamp_subsec_nanos().try_into().unwrap();

        grpc_api::reminder::Task {
            id: self.id.to_string(),
            title: self.title,
            remind_at: Some(Timestamp { seconds, nanos }),
            who: Some(self.who.into()),
        }
    }
}

pub(crate) trait TaskRepository {
    async fn create(
        &self,
        id: Id,
        title: String,
        remind_at: DateTime<Utc>,
        who: UserIdentifier,
    ) -> Result<Task, ReminderError>;
    async fn get(&self, id: Id) -> Result<Task, ReminderError>;
    async fn list(
        &self,
        who: Option<UserIdentifier>,
        duration: Option<Duration>,
    ) -> Result<Vec<Task>, ReminderError>;
    async fn delete(&self, id: Id) -> Result<Task, ReminderError>;
    async fn update(
        &self,
        id: Id,
        title: Option<String>,
        remind_at: Option<DateTime<Utc>>,
    ) -> Result<Task, ReminderError>;
}
