#![feature(stmt_expr_attributes)]

use anyhow::Result;
use colored::Colorize;
use driver::grpc_api::serve;
use init::init_db;

use crate::init::init_notification_cache;

mod config;
mod domain;
mod driver;
mod endpoint;
mod init;
mod misc;
mod service;

#[tokio::main]
async fn main() -> Result<()> {
    #[rustfmt::skip]
    println!(
        "\n{}\n{}\n{}\n{}\n{}\n\nDon't Rest Yet Reminder API\n",
        r" ____  ______   __  ____                _           _           ".bold().magenta(),
        r"|  _ \|  _ \ \ / / |  _ \ ___ _ __ ___ (_)_ __   __| | ___ _ __ ".bold().magenta(),
        r"| | | | |_) \ V /  | |_) / _ \ '_ ` _ \| | '_ \ / _` |/ _ \ '__|".bold().magenta(),
        r"| |_| |  _ < | |   |  _ <  __/ | | | | | | | | | (_| |  __/ |   ".bold().magenta(),
        r"|____/|_| \_\|_|   |_| \_\___|_| |_| |_|_|_| |_|\__,_|\___|_|   ".bold().magenta(),
    );
    log!("BOOT" -> "Start initialize...".green());

    init_db().await?;
    init_notification_cache().await?;

    log!("BOOT" -> "Initialize is finished".green());

    serve().await?;

    Ok(())
}
