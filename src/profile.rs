use crate::config::Config;
use crate::error::RetoroError;
use crate::utils::{deserialize_peer_id, serialize_peer_id};
use libp2p::{identity::Keypair, Multiaddr, PeerId};
use openssl::pkey::{PKey, Private};
use serde::{Deserialize, Serialize};
use std::fs;

type Name = String;

#[derive(Clone, Debug)]
pub struct Profile {
    name: Name,
    key: PKey<Private>,
    known_users: Vec<UserProfile>,
    known_networks: Vec<String>,
}

impl Profile {
    /// Creates a new profile from provided name and generates new keypair
    fn new_from_key(name: String, key: PKey<Private>) -> Self {
        Profile {
            name,
            key,
            known_users: vec![],
            known_networks: vec![],
        }
    }

    /// Creates a new profile from provided name while generating key
    pub fn new(name: String) -> Result<Self, RetoroError> {
        let key = openssl::pkey::PKey::generate_ed25519()?;
        Ok(Profile::new_from_key(name, key))
    }

    fn read_key_from_file(path: &str) -> Result<PKey<Private>, RetoroError> {
        let pem = fs::read(path)?;
        Ok(openssl::pkey::PKey::private_key_from_pem(&pem)?)
    }

    pub fn load_from_config(config: &Config) -> Result<Profile, RetoroError> {
        let name = config.get_name();
        let path = config.get_pem_file_path();

        if let (Some(name), Some(path)) = (name, path) {
            let key = Profile::read_key_from_file(&path)?;
            Ok(Profile::new_from_key(name, key))
        } else {
            Err(RetoroError::InvalidProfile)
        }
    }

    pub fn write_key_to_file(&self, path: &str) -> Result<(), RetoroError> {
        let pem = self.key.private_key_to_pem_pkcs8()?;
        fs::write(path, pem)?;
        Ok(())
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn known_users(&self) -> &[UserProfile] {
        &self.known_users
    }

    pub fn known_networks(&self) -> &[String] {
        &self.known_networks
    }

    pub fn keypair(&self) -> Result<Keypair, RetoroError> {
        let mut key_bytes = self.key.raw_private_key()?;
        Ok(Keypair::ed25519_from_bytes(&mut key_bytes)?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct UserProfile {
    names: Vec<Name>,
    #[serde(
        serialize_with = "serialize_peer_id",
        deserialize_with = "deserialize_peer_id"
    )]
    id: PeerId,
    nodes: Vec<Multiaddr>,
}

#[cfg(test)]
mod test {
    use libp2p::identity::Keypair;

    use crate::error::RetoroError;

    use super::Profile;

    #[test]
    fn test() -> Result<(), RetoroError> {
        let key = openssl::pkey::PKey::generate_ed25519()?;
        let mut key_bytes = key.raw_private_key()?;
        let keypair = Keypair::ed25519_from_bytes(&mut key_bytes)?;
        let name = "Somename".to_string();
        let user = Profile::new_from_key(name, key.clone());

        assert_eq!(user.name(), "Somename");
        assert_eq!(user.keypair().unwrap().public(), keypair.public());
        assert_eq!(user.known_users().len(), 0);
        assert_eq!(user.known_networks().len(), 0);
        Ok(())
    }
}
