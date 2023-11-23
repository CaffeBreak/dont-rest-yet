use std::env;

use anyhow::Result;
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use surrealdb::{engine::any::Any, opt::auth::Root, Surreal};

use crate::{
    config::Config, driver::task_repository::TaskRepositorySurrealDriver, log,
    service::service::TaskService,
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
    }
});
pub(crate) static DB: Lazy<Surreal<Any>> = Lazy::new(|| Surreal::init());
pub(crate) static TASK_SERVICE: Lazy<TaskService<TaskRepositorySurrealDriver>> =
    Lazy::new(|| TaskService {
        task_repo: TaskRepositorySurrealDriver,
    });

pub(crate) async fn init_db() -> Result<()> {
    log!("INFO" | "Connect to {} ...", CONFIG.db_uri);

    DB.connect(CONFIG.db_uri.to_string()).await?;
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    DB.use_ns("dry").use_db("reminder").await?;

    log!("INFO" -> "Connected.");

    Ok(())
}
