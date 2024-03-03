use libp2p::{multiaddr::Protocol, Multiaddr};
use log::debug;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, Ipv6Addr};

use crate::error::RetoroError;
use std::fs::read_to_string;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Configuration section related to profile
    /// It is only prividing the most basic information
    profile: ProfileConfig,
    /// Configuration section related to node
    /// It specifies the
    node: NodeConfig,
}

#[derive(Debug, Serialize, Deserialize)]
enum InterfaceMode {
    IpV4,
    IpV6,
    Both,
}

#[derive(Debug, Serialize, Deserialize)]
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
        let content = read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
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
                interface_mode: InterfaceMode::IpV4,
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
            interface_mode = \"IpV4\"
            port = 5511
        ";
        let a: Config = toml::from_str(c).unwrap();

        dbg!(ser_cfg);
        dbg!(a);
    }
}
