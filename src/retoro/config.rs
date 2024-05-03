use std::net::Ipv4Addr;

use ed25519_dalek::SigningKey;
use libp2p::{multiaddr::Protocol, Multiaddr};
use rand::rngs::OsRng;

#[derive(Debug, Eq, PartialEq)]
pub struct Config {
    /// Name of user
    name: String,
    /// Keypair
    keypair: [u8; 64],
    /// Determine wehter to listen on ipv6 or ipv4 loopback address, the default is ipv4
    interfaces: Vec<Multiaddr>,
    /// Bootnode list
    bootnodes: Vec<Multiaddr>,
}

impl Config {
    pub fn new(
        name: String,
        keypair: [u8; 64],
        interfaces: Vec<Multiaddr>,
        bootnodes: Vec<Multiaddr>,
    ) -> Self {
        Self {
            name,
            keypair,
            interfaces,
            bootnodes,
        }
    }

    pub fn bootnodes(&self) -> Vec<Multiaddr> {
        self.bootnodes.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn keypair(&self) -> [u8; 64] {
        self.keypair
    }

    pub fn interfaces(&self) -> Vec<Multiaddr> {
        self.interfaces.clone()
    }

    pub fn default() -> Self {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let keypair = signing_key.to_keypair_bytes();
        let public = signing_key.verifying_key().to_bytes();
        let public_string = String::from_utf8(public.to_vec()).unwrap();
        let profile_name = format!("Node-{public_string}");
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
            interfaces,
            bootnodes: vec![],
        }
    }
}
