use libp2p::{multiaddr::Protocol, Multiaddr};
use log::debug;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, Ipv6Addr};

use crate::error::RetoroError;
use std::fs::read_to_string;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Config {
    /// Configuration section related to profile
    /// It is only prividing the most basic information
    profile: ProfileConfig,
    /// Configuration section related to node
    /// It specifies the
    node: NodeConfig,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
enum InterfaceMode {
    IpV4,
    IpV6,
    Both,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct NodeConfig {
    /// Determine wehter to listen on ipv6 or ipv4 loopback address, the default is ipv4
    interface_mode: InterfaceMode,
    /// The port used to listen on all interfaces
    port: u16,
    /// Bootnode list
    bootnodes: Vec<Multiaddr>,
}

impl Config {
    pub fn new_from_file(path: &str) -> Result<Self, RetoroError> {
        let content = read_to_string(path)
            .map_err(|e| RetoroError::Config(format!("Failed reading file: {e}")))?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| RetoroError::Config(format!("Failed parsing config file: {e}")))?;
        debug!("parsed config: \n{:#?}", config);
        Ok(config)
    }

    pub fn get_addrs(&self) -> Vec<Multiaddr> {
        let interfaces = match self.node.interface_mode {
            InterfaceMode::IpV6 => vec![Protocol::from(Ipv6Addr::UNSPECIFIED)],
            InterfaceMode::IpV4 => vec![Protocol::from(Ipv4Addr::UNSPECIFIED)],
            InterfaceMode::Both => vec![
                Protocol::from(Ipv4Addr::UNSPECIFIED),
                Protocol::from(Ipv6Addr::UNSPECIFIED),
            ],
        };

        let mut addrs = vec![];
        interfaces.into_iter().for_each(|p| {
            addrs.push(
                Multiaddr::empty()
                    .with(p.clone())
                    .with(Protocol::Tcp(self.node.port)),
            );
            addrs.push(
                Multiaddr::empty()
                    .with(p)
                    .with(Protocol::Udp(self.node.port))
                    .with(Protocol::QuicV1),
            );
        });
        addrs
    }

    pub fn get_bootnodes(&self) -> Vec<Multiaddr> {
        self.node.bootnodes.clone()
    }

    pub fn get_name(&self) -> String {
        self.profile.profile_name.to_owned()
    }

    pub fn get_pem_file_path(&self) -> String {
        self.profile.pem_file.to_owned()
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct ProfileConfig {
    /// Name of user
    profile_name: String,
    /// Path to file storing the keys
    pem_file: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_config() {
        let cfg = default_config();
        let c = "
            [profile]
            profile_name = \"profile\"
            pem_file = \"privatekey.pem\"

            [node]
            bootnodes = []
            interface_mode = \"IpV4\"
            port = 5511
        ";
        let a: Config = toml::from_str(c).unwrap();

        assert_eq!(cfg, a);
    }

    fn default_config() -> Config {
        Config {
            node: NodeConfig {
                bootnodes: vec![],
                interface_mode: InterfaceMode::IpV4,
                port: 5511,
            },
            profile: ProfileConfig {
                pem_file: "privatekey.pem".to_string(),
                profile_name: "profile".to_string(),
            },
        }
    }
}
