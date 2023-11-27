use std::time::Duration;

use async_stream::stream;
use chrono::Utc;
use tokio::time;
use tokio_stream::Stream;

use crate::domain::task::{Task, TaskRepository};

use super::service::NotificationService;

impl<T: TaskRepository> NotificationService<T> {
    pub(crate) fn send_notification(&self) -> impl Stream<Item = Task> + '_ {
        stream! {
            let mut interval = time::interval(Duration::from_secs(10));

            loop {
                {
                    let mut task_cache = match self.task_cache.try_lock() {
                        Ok(locked_task) => locked_task,
                        Err(_) => continue,
                    };
                    if task_cache.len() < 1 {
                        continue;
                    }

                    while task_cache.len() > 0 && (task_cache[0].remind_at - Utc::now()).num_minutes() == 0 {
                        yield task_cache.pop().unwrap();
                    }
                }

                interval.tick().await;
            }
        }
    }
}
