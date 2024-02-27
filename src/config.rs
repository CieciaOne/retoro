use libp2p::{multiaddr::Protocol, Multiaddr};
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, Ipv6Addr};
use tracing::debug;

use crate::error::RetoroError;
use std::fs::read_to_string;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Configuration section related to profile
    profile: ProfileConfig,
    /// Configuration section related to node
    node: NodeConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct NodeConfig {
    /// Determine wehter to listen on ipv6 or ipv4 loopback address, the default is ipv4
    use_ipv6: bool,
    /// The port used to listen on all interfaces
    port: u16,
    /// Bootnode list
    bootnodes: Vec<Multiaddr>,
}

impl Config {
    pub fn new_from_file(path: &str) -> Result<Self, RetoroError> {
        let content = read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        debug!("parsed config: \n{:?}", config);
        Ok(config)
    }

    pub fn get_tcp_addrs(&self) -> Multiaddr {
        Multiaddr::empty()
            .with(match self.node.use_ipv6 {
                true => Protocol::from(Ipv6Addr::UNSPECIFIED),
                false => Protocol::from(Ipv4Addr::UNSPECIFIED),
            })
            .with(Protocol::Tcp(self.node.port))
    }

    pub fn get_quic_addrs(&self) -> Multiaddr {
        Multiaddr::empty()
            .with(match self.node.use_ipv6 {
                true => Protocol::from(Ipv6Addr::UNSPECIFIED),
                false => Protocol::from(Ipv4Addr::UNSPECIFIED),
            })
            .with(Protocol::Udp(self.node.port))
            .with(Protocol::QuicV1)
    }

    pub fn get_bootnodes(&self) -> Vec<Multiaddr> {
        debug!("got bootnodes: {:?}", self.node.bootnodes);
        self.node.bootnodes.clone()
    }

    pub fn get_name(&self) -> Option<String> {
        self.profile.profile_name.to_owned()
    }

    pub fn get_pem_file_path(&self) -> Option<String> {
        self.profile.pem_file.to_owned()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ProfileConfig {
    /// Name of user
    profile_name: Option<String>,
    /// Path to file storing the keys
    pem_file: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cfg = Config {
            node: NodeConfig {
                bootnodes: vec![],
                use_ipv6: false,
                port: 1,
            },
            profile: ProfileConfig {
                pem_file: Some("privatekey.pem".to_string()),
                profile_name: Some("profile".to_string()),
            },
        };
        let ser_cfg = toml::to_string(&cfg).unwrap();

        let c = "
            [profile]
            profile_name = \"profile\"
            pem_file = \"privatekey.pem\"

            [node]
            bootnodes = []
            use_ipv6 = false
            port = 5511
        ";
        let a: Config = toml::from_str(c).unwrap();

        dbg!(ser_cfg);
        dbg!(a);
    }
}
