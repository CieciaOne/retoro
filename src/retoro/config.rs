use libp2p::{identity::Keypair, multiaddr::Protocol, Multiaddr};
use std::net::Ipv4Addr;

use super::error::Error;

#[derive(Debug)]
pub struct Config {
    /// Name of user
    name: String,
    /// Keypair
    keypair: Keypair,
    /// Determine wehter to listen on ipv6 or ipv4 loopback address, the default is ipv4
    addresses: Vec<Multiaddr>,
    /// Bootnode list
    bootnodes: Vec<Multiaddr>,
}

impl Config {
    pub fn new(
        name: String,
        keypair: Keypair,
        interfaces: Vec<Multiaddr>,
        bootnodes: Vec<Multiaddr>,
    ) -> Result<Self, Error> {
        if interfaces.is_empty() {
            return Err(Error::Config(
                "At least one interface is required.".to_string(),
            ));
        }
        Ok(Self {
            name,
            keypair,
            addresses: interfaces,
            bootnodes,
        })
    }

    #[allow(unused)]
    fn new_from_key(name: String, keypair: Keypair) -> Self {
        Config {
            name,
            keypair,
            addresses: vec![],
            bootnodes: vec![],
        }
    }

    pub fn bootnodes(&self) -> Vec<Multiaddr> {
        self.bootnodes.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn keypair(&self) -> Keypair {
        self.keypair.clone()
    }

    // pub fn ed25519_keypair(&self) -> Result<Ed25519Keypair, Error>{
    // Ed25519Keypair::from(self.keypair.)?
    // }

    pub fn interfaces(&self) -> Vec<Multiaddr> {
        self.addresses.clone()
    }
}
impl Default for Config {
    fn default() -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let profile_name = format!("Node-{id}");
        let keypair = Keypair::generate_ed25519();
        let interfaces = vec![
            Multiaddr::empty()
                .with(Protocol::from(Ipv4Addr::UNSPECIFIED))
                .with(Protocol::Tcp(5511)),
            Multiaddr::empty()
                .with(Protocol::from(Ipv4Addr::UNSPECIFIED))
                .with(Protocol::Udp(5511))
                .with(Protocol::QuicV1),
        ];

        Config {
            name: profile_name,
            keypair,
            addresses: interfaces,
            bootnodes: vec![],
        }
    }
}
