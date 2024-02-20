use libp2p::{identity::Keypair, PeerId};

#[derive(Clone, Debug)]
pub struct Profile {
    name: String,
    keypair: Keypair,
    known_users: Vec<PeerId>,
    known_networks: Vec<String>,
}

impl Profile {
    /// Creates a new profile from provided name and  keypair
    pub fn new_from_keypair(name: String, keypair: Keypair) -> Self {
        Profile {
            name,
            keypair: keypair,
            known_users: vec![],
            known_networks: vec![],
        }
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn known_users(&self) -> &[PeerId] {
        &self.known_users
    }

    pub fn known_networks(&self) -> &[String] {
        &self.known_networks
    }

    pub fn keypair(&self) -> Keypair {
        self.keypair.clone()
    }
}

#[cfg(test)]
mod test {
    use libp2p::identity::Keypair;

    use super::Profile;

    #[test]
    fn test() {
        let keypair = Keypair::generate_ed25519();
        let name = "Somename".to_string();
        let user = Profile::new_from_keypair(name, keypair.clone());

        assert_eq!(user.name(), "Somename");
        assert_eq!(user.keypair().public(), keypair.public());
        assert_eq!(user.known_users().len(), 0);
        assert_eq!(user.known_networks().len(), 0);
    }
}
