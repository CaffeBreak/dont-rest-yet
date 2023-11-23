use crate::{domain::task::TaskRepository, log, misc::id::Id};
use anyhow::{Ok, Result};
use chrono_tz::Asia;

use super::service::TaskService;

impl<T: TaskRepository> TaskService<T> {
    pub async fn delete_task(&self, id: Id) -> Result<()> {
        let deleted = self.task_repo.delete(id).await?;

        let message = format!(
            "以下のタスクを削除しました:\nid: {}\nタスクの内容: {}\nリマインド時間: {}\nリマインド対象の識別子: {}",
            deleted.id.to_string(),
            deleted.title,
            deleted.remind_at.with_timezone(&Asia::Tokyo).format("%Y/%M/%d %T"),
            deleted.who.id
        );
        log!("INFO" -> message);

        Ok(())
    }
}
