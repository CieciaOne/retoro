
use std::str::FromStr;

use retoro::{self, Keypair};
pub async fn start() -> anyhow::Result<()>{
    let mut retoro = retoro::Node::new()?;
    // let keypair = Keypair::generate_ed25519();
    // let addr = retoro::Multiaddr::from_str("/ip4/127.0.0.1/tcp/5511")?;
    // let interfaces = vec![addr];
    // let bootnodes = vec![];
    // let config = retoro::Config::new("N".to_string(), keypair, interfaces, bootnodes)?;
    // let mut retoro = retoro::Node::with_config(config)?;
    retoro.run().await?;
    Ok(())
}
    // fn read_key_from_file(path: &str) -> Result<Keys, Error> {
    //     match Keys::read_pkcs8_pem_file(path) {
    //         Ok(key) => Ok(key),
    //         Err(e) => Err(Error::Keypair(format!(
    //             "Error occured when loading keypair {path}: {e}"
    //         ))),
    //     }
    // }

    // /// load profile specified in config
    // pub fn load_from_config(config: &Config) -> Result<Data, Error> {
    //     let name = config.name();
    //     let path = config.keypair();

    //     if name.is_empty() {
    //         return Err(Error::Data("Missing name in config".to_string()));
    //     }
    //     if path.is_empty() {
    //         return Err(Error::Data(
    //             "Missing pem file path in config".to_string(),
    //         ));
    //     }

    //     let key = Data::read_key_from_file(&path)?;
    //     Ok(Data::new_from_key(name, key))
    // }

    // #[allow(unused)]
    // pub fn write_key_to_file(&self, path: &str) -> Result<(), Error> {
    //     use ed25519_dalek::pkcs8::spki::der::pem::LineEnding;
    //     match self.key.write_pkcs8_pem_file(path, LineEnding::CR) {
    //         Ok(_) => Ok(()),
    //         Err(e) => Err(Error::Keypair(format!(
    //             "Error occured when writing keypair {path}: {e}"
    //         ))),
    //     }
    // }
