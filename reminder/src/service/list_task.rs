use crate::{
    domain::{
        task::{Task, TaskRepository},
        user::User,
    },
    log,
};
use anyhow::{Ok, Result};
use chrono_tz::Asia;

use super::service::TaskService;

impl<T: TaskRepository> TaskService<T> {
    pub async fn list_task(&self, who: Option<User>) -> Result<Vec<Task>> {
        let list = self.task_repo.list(who).await?;

        let message = format!(
            "以下のタスクを取得しました:\n{}",
            list.iter().map(|task| format!(
                "{{\tid: {}\n\tタスクの内容: {}\n\tリマインド時間: {}\n\tリマインド対象の識別子: {}\n}}\n",
                task.id.to_string(),
                task.title,
                task.remind_at
                    .with_timezone(&Asia::Tokyo)
                    .format("%Y/%M/%d %T"),
                task.who.id
            )).collect::<String>()
        );
        log!("INFO" -> message);

        Ok(list)
    }
}
