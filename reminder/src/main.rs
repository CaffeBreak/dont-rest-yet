use anyhow::{Ok, Result};
use api::serve;

pub mod api;
pub mod misc;

#[tokio::main]
async fn main() -> Result<()> {
    serve().await?;

    Ok(())
}
