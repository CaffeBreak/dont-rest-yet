#![feature(stmt_expr_attributes)]

use anyhow::{Ok, Result};
use colored::Colorize;
use driver::grpc_api::serve;
use init::init_db;

pub mod config;
pub mod domain;
pub mod driver;
pub mod init;
pub mod misc;
pub mod service;

#[tokio::main]
async fn main() -> Result<()> {
    #[rustfmt::skip]
    println!(
        "\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n\n",
        r" ____              _ _     ____           _    __   __   _   ".bold().magenta(),
        r"|  _ \  ___  _ __ ( ) |_  |  _ \ ___  ___| |_  \ \ / /__| |_ ".bold().magenta(),
        r"| | | |/ _ \| '_ \|/| __| | |_) / _ \/ __| __|  \ V / _ \ __|".bold().magenta(),
        r"| |_| | (_) | | | | | |_  |  _ <  __/\__ \ |_    | |  __/ |_ ".bold().magenta(),
        r"|____/ \___/|_| |_|__\__| |_| \_\___||___/\__|   |_|\___|\__|".bold().magenta(),
        r"                |  _ \ ___ _ __ ___ (_)_ __   __| | ___ _ __ ".blue(),
        r"                | |_) / _ \ '_ ` _ \| | '_ \ / _` |/ _ \ '__|".blue(),
        r"                |  _ <  __/ | | | | | | | | | (_| |  __/ |   ".blue(),
        r"                |_| \_\___|_| |_| |_|_|_| |_|\__,_|\___|_|   ".blue()
    );

    init_db().await?;

    serve().await?;

    Ok(())
}
