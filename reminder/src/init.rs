use std::env;

use anyhow::{bail, Context, Result};
use chrono::Duration;
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use surrealdb::{engine::any::Any, opt::auth::Root, Surreal};
use tokio::time;

use crate::{
    config::Config,
    domain::task::TaskRepository,
    driver::task_repository::TaskRepositorySurrealDriver,
    log,
    misc::error::ReminderError,
    service::service::{NotificationService, TaskService},
};

pub(crate) static CONFIG: Lazy<Config> = Lazy::new(|| {
    let _ = dotenv();
    let default = Config::default();

    Config {
        grpc_port: match env::var("GRPC_PORT") {
            Ok(port_str) => port_str
                .parse()
                .expect(format!("{} is invalid port number", port_str).as_str()),
            Err(_) => default.grpc_port,
        },
        db_uri: env::var("DB_URI").unwrap_or(default.db_uri),
        db_user: env::var("DB_USER").unwrap_or(default.db_user),
        db_pass: env::var("DB_PASS").unwrap_or(default.db_pass),
        notification_cache_interval: match env::var("NOTIFICATION_CACHE_INVETVAL") {
            Ok(interval_str) => interval_str.parse().expect(
                format!(
                    "{} is invalid value for notification cache interval",
                    interval_str
                )
                .as_str(),
            ),
            Err(_) => default.notification_cache_interval,
        },
    }
});
pub(crate) static DB: Lazy<Surreal<Any>> = Lazy::new(|| Surreal::init());
pub(crate) static TASK_SERVICE: Lazy<TaskService<TaskRepositorySurrealDriver>> =
    Lazy::new(|| TaskService {
        task_repo: TaskRepositorySurrealDriver,
    });
pub(crate) static NOTIFICATION_SERVICE: Lazy<NotificationService<TaskRepositorySurrealDriver>> =
    Lazy::new(|| NotificationService::new(TaskRepositorySurrealDriver));

pub(crate) async fn init_db() -> Result<()> {
    log!("DB" -> format!("Connect to {} ...", CONFIG.db_uri).magenta());

    DB.connect(CONFIG.db_uri.to_string())
        .await
        .context(format!("Failed to connect DB with URI: {}.", CONFIG.db_uri))?;
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .context("Signin is failed.")?;
    DB.use_ns("dry")
        .use_db("reminder")
        .await
        .context("Failed to use Namespace or Database.")?;

    log!("DB" -> format!("Database connected").magenta());

    Ok(())
}

pub(crate) async fn init_notification_cache() -> Result<()> {
    log!("INFO" -> "Initialize notification cache.".yellow());

    // 定期的にキャッシュを更新する
    let mut interval = time::interval(time::Duration::from_secs(
        CONFIG.notification_cache_interval as u64 * 60,
    ));
    tokio::spawn(async move {
        log!("INFO" -> "Start cache refreshing...".yellow());

        let tasks = match NOTIFICATION_SERVICE
            .task_repo
            .list(
                None,
                Some(Duration::minutes(
                    (CONFIG.notification_cache_interval * 3).into(),
                )),
            )
            .await
        {
            Ok(tasks) => tasks,
            Err(error) => match error {
                ReminderError::DBOperationError(error) => {
                    log!("ERROR" -> "Notification cache failed.".bold().red());
                    log!("ERROR" -> format!("Reason: {}", error.to_string()).bold().red());

                    bail!("Notification cache failed.");
                }
                ReminderError::TaskNotFound { id: _ } => return Ok(()),
            },
        };
        {
            let mut cache = NOTIFICATION_SERVICE.task_cache.lock().await;
            *cache = tasks;
        }

        log!("INFO" -> "cache refreshing is finished.".yellow());
        interval.tick().await;

        Ok(())
    });

    log!("INFO" -> "Notifications is cached.".yellow());

    Ok(())
}
