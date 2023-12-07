use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::any::Any, method::Update, sql::Thing};

use crate::{
    domain::{
        task::{Task, TaskRepository},
        user::UserIdentifier,
    },
    init::DB,
    log,
    misc::{error::ReminderError, id::Id},
};

use super::user_repository::UserRecord;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct TaskRecord {
    id: Thing,
    title: String,
    remind_at: DateTime<Utc>,
    who: Thing,
}
impl From<Task> for TaskRecord {
    fn from(value: Task) -> Self {
        Self {
            id: Thing::from(("task".to_string(), value.id.to_string())),
            title: value.title,
            remind_at: value.remind_at,
            who: Thing::from((
                "user".to_string(),
                Into::<surrealdb::sql::Id>::into(value.who),
            )),
        }
    }
}
impl TryInto<Task> for TaskRecord {
    type Error = ReminderError;

    fn try_into(self) -> Result<Task, Self::Error> {
        let user_identifier = self.who.id.try_into().unwrap();

        Ok(Task {
            id: self.id.id.to_string().into(),
            title: self.title,
            remind_at: self.remind_at,
            who: user_identifier,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TaskUpdate {
    title: String,
    remind_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TaskTitleUpdate {
    title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TaskRemindAtUpdate {
    remind_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct TaskRecordWithUser {
    id: Thing,
    title: String,
    remind_at: DateTime<Utc>,
    who: UserRecord,
}
impl Into<Task> for TaskRecordWithUser {
    fn into(self) -> Task {
        Task {
            id: Id::from(self.id.id.to_string()),
            title: self.title,
            remind_at: self.remind_at,
            who: self.who.id.id.try_into().unwrap(),
        }
    }
}

pub(crate) struct TaskRepositorySurrealDriver;

impl TaskRepository for TaskRepositorySurrealDriver {
    async fn create(
        &self,
        id: Id,
        title: String,
        remind_at: DateTime<Utc>,
        who: UserIdentifier,
    ) -> Result<Task, ReminderError> {
        let created: TaskRecord = DB
            .create("task")
            .content(TaskRecord {
                id: Thing::from(("task".to_string(), id.to_string())),
                title: title.clone(),
                remind_at,
                who: Thing::from(("user".to_string(), Into::<surrealdb::sql::Id>::into(who))),
            })
            .await
            .map_err(|error| ReminderError::DBOperationError(error))?
            .pop()
            .unwrap();
        // let created = self.get(created.id.id.to_string().into()).await?;
        log!("DEBUG" -> format!("Created: {:?}", created).dimmed());

        created.try_into()
    }

    async fn get(&self, id: Id) -> Result<Task, ReminderError> {
        let query = format!(
            "select * from {} fetch who",
            Thing::from(("task".to_string(), id.to_string()))
        );
        let task: Option<TaskRecordWithUser> = DB
            .query(query)
            .await
            .map_err(|error| ReminderError::DBOperationError(error))?
            .take(0)
            .map_err(|error| ReminderError::DBOperationError(error))?;
        let task = task.ok_or(ReminderError::TaskNotFound { id: id.to_string() })?;
        log!("DEBUG" -> format!("Got: {:?}", task).dimmed());

        Ok(task.into())
    }

    async fn list(
        &self,
        who: Option<UserIdentifier>,
        duration: Option<Duration>,
    ) -> Result<Vec<Task>, ReminderError> {
        let query = "select * from task".to_string();
        let query = match (who, duration) {
            (None, None) => query,
            (None, Some(duration)) => {
                let dt_now = Utc::now();
                let end_time = dt_now + duration;

                format!(
                    "{} where \"{}\" <= remind_at && remind_at <= \"{}\"",
                    query,
                    dt_now.to_rfc3339(),
                    end_time.to_rfc3339()
                )
            }
            (Some(who), None) => {
                format!(
                    "{} where who = {}",
                    query,
                    Thing::from(("user".to_string(), Into::<surrealdb::sql::Id>::into(who)))
                )
            }
            (Some(who), Some(duration)) => {
                let dt_now = Utc::now();
                let end_time = dt_now + duration;

                format!(
                    "{} where \"{}\" <= remind_at && remind_at <= \"{}\" && who == \"{}\"",
                    query,
                    dt_now.to_rfc3339(),
                    end_time.to_rfc3339(),
                    Thing::from(("user".to_string(), Into::<surrealdb::sql::Id>::into(who))),
                )
            }
        };
        let query = format!("{} fetch who", query);

        let list: Vec<TaskRecordWithUser> = DB
            .query(query)
            .await
            .map_err(|error| ReminderError::DBOperationError(error))?
            .take(0)
            .map_err(|error| ReminderError::DBOperationError(error))?;
        log!("DEBUG" -> format!("Listed: {:?}", list).dimmed());

        Ok(list.into_iter().map(|task| task.into()).collect())
    }

    async fn delete(&self, id: Id) -> Result<Task, ReminderError> {
        let deleted: TaskRecord = DB
            .delete(("task", id.to_string()))
            .await
            .map_err(|error| ReminderError::DBOperationError(error))?
            .unwrap();
        log!("DEBUG" -> format!("Deleted: {:?}", deleted).dimmed());

        deleted.try_into()
    }

    async fn update(
        &self,
        id: Id,
        title: Option<String>,
        remind_at: Option<DateTime<Utc>>,
    ) -> Result<Task, ReminderError> {
        let update: Update<'_, Any, Option<TaskRecord>> = DB.update(("task", id.to_string()));

        let updated = match (title, remind_at) {
            (None, None) => todo!(),
            (None, Some(remind_at)) => update.merge(TaskRemindAtUpdate { remind_at }).await,
            (Some(title), None) => update.merge(TaskTitleUpdate { title }).await,
            (Some(title), Some(remind_at)) => update.merge(TaskUpdate { title, remind_at }).await,
        }
        .map_err(|error| ReminderError::DBOperationError(error))?
        .unwrap();
        // let updated = self.get(updated.id.id.to_string().into()).await?;
        log!("DEBUG" -> format!("Updated: {:?}", updated).dimmed());

        updated.try_into()
    }
}
