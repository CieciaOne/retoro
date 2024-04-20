use crate::config::Config;
use crate::error::RetoroError;
use crate::utils::{deserialize_peer_id, serialize_peer_id};
use ed25519_dalek::{pkcs8::DecodePrivateKey, pkcs8::EncodePrivateKey, SigningKey};
use libp2p::{identity::Keypair, Multiaddr, PeerId};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

type Name = String;
type Keys = SigningKey;
#[derive(Clone, Debug)]
pub struct Profile {
    name: Name,
    key: Keys,
    known_users: Vec<UserProfile>,
    known_networks: Vec<String>,
}

impl Profile {
    /// Creates a new profile from provided name and generates new keypair
    fn new_from_key(name: String, key: Keys) -> Self {
        Profile {
            name,
            key,
            known_users: vec![],
            known_networks: vec![],
        }
    }

    /// Creates a new profile from provided name while generating key
    #[allow(unused)]
    pub fn new(name: String) -> Result<Self, RetoroError> {
        let mut rng = OsRng;
        let key = Keys::generate(&mut rng);
        Ok(Profile::new_from_key(name, key))
    }

    fn read_key_from_file(path: &str) -> Result<Keys, RetoroError> {
        match Keys::read_pkcs8_pem_file(path) {
            Ok(key) => Ok(key),
            Err(e) => Err(RetoroError::Keypair(format!(
                "Error occured when loading keypair {path}: {e}"
            ))),
        }
    }

    /// load profile specified in config
    pub fn load_from_config(config: &Config) -> Result<Profile, RetoroError> {
        let name = config.get_name();
        let path = config.get_pem_file_path();

        if name.is_empty() {
            return Err(RetoroError::Profile("Missing name in config".to_string()));
        }
        if path.is_empty() {
            return Err(RetoroError::Profile(
                "Missing pem file path in config".to_string(),
            ));
        }

        let key = Profile::read_key_from_file(&path)?;
        Ok(Profile::new_from_key(name, key))
    }

    #[allow(unused)]
    pub fn write_key_to_file(&self, path: &str) -> Result<(), RetoroError> {
        use ed25519_dalek::pkcs8::spki::der::pem::LineEnding;
        match self.key.write_pkcs8_pem_file(path, LineEnding::CR) {
            Ok(_) => Ok(()),
            Err(e) => Err(RetoroError::Keypair(format!(
                "Error occured when writing keypair {path}: {e}"
            ))),
        }
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    #[allow(unused)]
    pub fn known_users(&self) -> &[UserProfile] {
        &self.known_users
    }

    #[allow(unused)]
    pub fn known_networks(&self) -> &[String] {
        &self.known_networks
    }

    pub fn keypair(&self) -> Result<Keypair, RetoroError> {
        let mut key_bytes = self.key.to_bytes();
        let keypair = Keypair::ed25519_from_bytes(&mut key_bytes)
            .map_err(|e| RetoroError::Keypair(format!("Failed decoding keypair: {e}")))?;
        Ok(keypair)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserProfile {
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
    use super::Keys;
    use libp2p::identity::Keypair;
    use rand::rngs::OsRng;

    use crate::error::RetoroError;

    use super::Profile;

    #[test]
    fn new_profile_from_keypair() -> Result<(), RetoroError> {
        let mut rng = OsRng;
        let key = Keys::generate(&mut rng);
        let mut key_bytes = key.to_bytes();
        let keypair = Keypair::ed25519_from_bytes(&mut key_bytes)
            .map_err(|e| RetoroError::Keypair(format!("Failed generating the keypair {e}")))?;
        let name = "Somename".to_string();
        let user = Profile::new_from_key(name, key.clone());

        assert_eq!(user.name(), "Somename");
        assert_eq!(user.keypair().unwrap().public(), keypair.public());
        assert_eq!(user.known_users().len(), 0);
        assert_eq!(user.known_networks().len(), 0);
        Ok(())
    }
}
