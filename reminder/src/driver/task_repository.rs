use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

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
        log!("DEBUG" | "Created: {:?}", created);

        Ok(created.pop().unwrap().into())
    }

    async fn get(&self, id: Id) -> Result<Task, ReminderError> {
        let task: TaskRecord = DB
            .select(("task", id.to_string()))
            .await
            .map_err(|error| ReminderError::DBOperationError(error))?
            .ok_or(ReminderError::TaskNotFound { id: id.to_string() })?;
        log!("DEBUG" | "Got: {:?}", task);

        Ok(task.into())
    }

    async fn list(&self, who: Option<User>) -> Result<Vec<Task>, ReminderError> {
        let list: Vec<TaskRecord> = match who {
            Some(who) => DB
                .query("select * from task where who = $who;")
                .bind(("who", who.id))
                .await
                .map_err(|error| ReminderError::DBOperationError(error))?
                .take(0)
                .unwrap(),
            None => DB
                .query("select * from task")
                .await
                .map_err(|error| ReminderError::DBOperationError(error))?
                .take(0)
                .unwrap(),
        };
        log!("DEBUG" | "Listed: {:?}", list);

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
        log!("DEBUG" | "Deleted: {:?}", deleted);

        Ok(deleted.into())
    }
}
