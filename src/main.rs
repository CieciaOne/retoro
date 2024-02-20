mod config;
mod error;
mod profile;
mod retoro;

use config::Config;
use retoro::Retoro;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let config = Config::new_from_file("./config.json").await?;

    let mut retoro = Retoro::new(config).await?;
    retoro.run().await?;

    Ok(())
}
