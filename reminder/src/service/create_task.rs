use anyhow::{Ok, Result};
use chrono::{DateTime, Utc};
use chrono_tz::Asia;

use crate::{
    domain::{
        task::{Task, TaskRepository},
        user::User,
    },
    log,
    misc::id::Id,
};

use super::service::TaskService;

impl<T: TaskRepository> TaskService<T> {
    pub async fn create_task(
        &self,
        title: String,
        remind_at: DateTime<Utc>,
        who: User,
    ) -> Result<Task> {
        let created = self
            .task_repo
            .create(Id::new(), title, remind_at, who)
            .await?;

        let message = format!("以下のタスクを追加しました:\nid: {}\nタスクの内容: {}\nリマインド時間: {}\nリマインド対象の識別子: {}", created.id.to_string(), created.title, created.remind_at.with_timezone(&Asia::Tokyo).format("%Y/%M/%d %T"), created.who.id);
        log!("INFO" -> message);

        Ok(created)
    }
}
