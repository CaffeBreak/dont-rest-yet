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
                let mut task_cache = self.task_cache.lock().await;
                if task_cache.len() > 0 {
                    let mut delete_flags: Vec<bool> = vec![];
                    for (i, task) in task_cache.iter().enumerate() {
                        delete_flags.push((task.remind_at - Utc::now()).num_seconds() < 30);
                        if delete_flags[i] {
                            yield task.clone();
                        }
                    }

                    let mut f = delete_flags.iter();
                    task_cache.retain(|_| !*f.next().unwrap());
                }

                interval.tick().await;
            }
        }
    }
}
