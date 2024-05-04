mod client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    client::start().await?;
    Ok(())
}
