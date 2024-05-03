
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let mut retoro = retoro::Node::new("Adi".to_string())?;
    retoro.run().await?;

    // let config = Config::new_from_file("./config.toml")?;
    // let profile = Profile::load_from_config(&config)?;

    // let mut retoro = Retoro::new(config, profile).await?;
    // retoro.run().await?;

    Ok(())
}
