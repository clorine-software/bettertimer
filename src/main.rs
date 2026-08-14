use anyhow::Result;

mod modules;
use modules::*;

#[tokio::main]
async fn main() -> Result<()> {
    modules::logic::logic().await?;

    Ok(())
}
