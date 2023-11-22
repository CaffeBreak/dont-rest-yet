use anyhow::{Ok, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{
    domain::{
        self,
        task::{Task, TaskRepository},
        user::User,
    },
    init::DB,
    log,
    misc::id::Id,
};

#[derive(Debug, Serialize, Deserialize)]
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
            id: Id::from_str(self.id.id.to_string()),
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
    ) -> Result<Task> {
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
            .await?;
        log!("DEBUG" | "Created: {:?}", created);

        Ok(TaskRecord::into(created.pop().unwrap()))
    }

    async fn list(&self, who: Option<User>) -> Result<Vec<domain::task::Task>> {
        todo!()
    }

    async fn delete(&self, id: Id) -> Result<()> {
        todo!()
    }
}
