use clap::Parser;
use libp2p::{multiaddr::Protocol, Multiaddr};
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, Ipv6Addr};
use tracing::debug;

use tokio::{fs::File, io::AsyncReadExt};

use crate::error::RetoroError;
#[derive(Debug, Parser, Serialize, Deserialize)]
// #[clap(name = "retoro", version, about)]
pub struct Config {
    /// Determine if the relay listen on ipv6 or ipv4 loopback address. the default is ipv4
    use_ipv6: bool,
    /// The port used to listen on all interfaces
    port: u16,
    /// Bootnode list
    bootnodes: Vec<Multiaddr>,
}

impl Config {
    pub async fn new_from_file(path: &str) -> Result<Self, RetoroError> {
        let mut file = File::open(path).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;

        debug!("read file contents: \n{}", content);
        let config: Config = serde_json::from_str(&content)?;

        debug!("parsed config: \n{:?}", config);
        Ok(config)
    }

    pub async fn new_from_url(url: &str) -> Result<Self, RetoroError> {
        todo!("implement fetching config from web");
    }

    pub fn get_tcp_addrs(&self) -> Multiaddr {
        Multiaddr::empty()
            .with(match self.use_ipv6 {
                true => Protocol::from(Ipv6Addr::UNSPECIFIED),
                false => Protocol::from(Ipv4Addr::UNSPECIFIED),
            })
            .with(Protocol::Tcp(self.port))
    }

    pub fn get_quic_addrs(&self) -> Multiaddr {
        Multiaddr::empty()
            .with(match self.use_ipv6 {
                true => Protocol::from(Ipv6Addr::UNSPECIFIED),
                false => Protocol::from(Ipv4Addr::UNSPECIFIED),
            })
            .with(Protocol::Udp(self.port))
            .with(Protocol::QuicV1)
    }

    pub fn get_bootnodes(&self) -> Vec<Multiaddr> {
        debug!("got bootnodes: {:?}", self.bootnodes);
        self.bootnodes.clone()
    }

    pub fn default() -> Self {
        Self {
            use_ipv6: false,
            port: 5511,
            bootnodes: vec![],
        }
    }
}
