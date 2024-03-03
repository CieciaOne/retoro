mod config;
mod error;
mod message;
mod profile;
mod retoro;
mod utils;

use config::Config;
use profile::Profile;
use retoro::Retoro;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let config = Config::new_from_file("./config.toml")?;
    let profile = Profile::load_from_config(&config)?;

    let mut retoro = Retoro::new(config, profile).await?;
    retoro.run().await?;

    Ok(())
}
