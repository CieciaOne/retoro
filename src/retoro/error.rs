use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Swarm error: {0}")]
    Swarm(String),
    #[error("Config error: {0}")]
    Config(String),
    #[error("Invalid profile: {0}")]
    Profile(String),
    #[error("Keypair error: {0}")]
    Keypair(String),
}
