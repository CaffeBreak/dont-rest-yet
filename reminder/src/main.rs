#![feature(stmt_expr_attributes)]

use anyhow::{Ok, Result};
use colored::Colorize;
use driver::grpc_api::serve;
use init::init_db;

pub mod config;
pub mod domain;
pub mod driver;
pub mod endpoint;
pub mod init;
pub mod misc;
pub mod service;

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

    init_db().await?;

    serve().await?;

    Ok(())
}
