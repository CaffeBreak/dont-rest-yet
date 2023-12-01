use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::any::Any, method::Update, sql::Thing};

use crate::{
    domain::{
        task::{Task, TaskRepository},
        user::User,
    },
    init::DB,
    log,
    misc::{error::ReminderError, id::Id},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TaskRecord {
    id: Thing,
    title: String,
    remind_at: DateTime<Utc>,
    who: String,
}
impl From<Task> for TaskRecord {
    fn from(value: Task) -> Self {
        Self {
            id: Thing {
                tb: "task".to_string(),
                id: surrealdb::sql::Id::String(value.id.to_string()),
            },
            title: value.title,
            remind_at: value.remind_at,
            who: value.who.id,
        }
    }
}
impl Into<Task> for TaskRecord {
    fn into(self) -> Task {
        Task {
            id: Id::from(self.id.id.to_string()),
            title: self.title,
            remind_at: self.remind_at,
            who: User { id: self.who },
        }
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

pub struct TaskRepositorySurrealDriver;

impl TaskRepository for TaskRepositorySurrealDriver {
    async fn create(
        &self,
        id: Id,
        title: String,
        remind_at: DateTime<Utc>,
        who: User,
    ) -> Result<Task, ReminderError> {
        let mut created: Vec<TaskRecord> = DB
            .create("task")
            .content(TaskRecord {
                id: Thing {
                    tb: "task".to_string(),
                    id: surrealdb::sql::Id::String(id.to_string()),
                },
                title: title.clone(),
                remind_at,
                who: who.id.clone(),
            })
            .await
            .map_err(|error| ReminderError::DBOperationError(error))?;
        log!("DEBUG" -> format!("Created: {:?}", created).dimmed());

        Ok(created.pop().unwrap().into())
    }

    async fn get(&self, id: Id) -> Result<Task, ReminderError> {
        let task: TaskRecord = DB
            .select(("task", id.to_string()))
            .await
            .map_err(|error| ReminderError::DBOperationError(error))?
            .ok_or(ReminderError::TaskNotFound { id: id.to_string() })?;
        log!("DEBUG" -> format!("Got: {:?}", task).dimmed());

        Ok(task.into())
    }

    async fn list(
        &self,
        who: Option<User>,
        duration: Option<Duration>,
    ) -> Result<Vec<Task>, ReminderError> {
        let query = "select * from task".to_string();
        let query = match (who, duration) {
            (None, None) => DB.query(query),
            (None, Some(duration)) => {
                let dt_now = Utc::now();
                let end_time = dt_now + duration;

                DB.query(format!(
                    "{} where remind_at >= $dt_now && remind_at <= $duration",
                    query
                ))
                .bind(("dt_now", dt_now))
                .bind(("duration", end_time))
            }
            (Some(who), None) => DB
                .query(format!("{} where who = $who", query))
                .bind(("who", who.id)),
            (Some(who), Some(duration)) => {
                let dt_now = Utc::now();
                let end_time = dt_now + duration;

                DB.query(format!(
                    "{} where remind_at >= $dt_now && remind_at <= $duration && who = $who",
                    query
                ))
                .bind(("dt_now", dt_now))
                .bind(("duration", end_time))
                .bind(("who", who.id))
            }
        };

        let list: Vec<TaskRecord> = query
            .await
            .map_err(|error| ReminderError::DBOperationError(error))?
            .take(0)
            .unwrap();
        log!("DEBUG" -> format!("Listed: {:?}", list).dimmed());

        Ok(list
            .iter()
            .map(|task| TaskRecord::into(task.clone()))
            .collect())
    }

    async fn delete(&self, id: Id) -> Result<Task, ReminderError> {
        let deleted: TaskRecord = DB
            .delete(("task", id.to_string()))
            .await
            .map_err(|error| ReminderError::DBOperationError(error))?
            .unwrap();
        log!("DEBUG" -> format!("Deleted: {:?}", deleted).dimmed());

        Ok(deleted.into())
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
        log!("DEBUG" -> format!("Updated: {:?}", updated).dimmed());

        Ok(updated.into())
    }
}
