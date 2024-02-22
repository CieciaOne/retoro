use chrono::Utc;
use libp2p::{multihash::Multihash, PeerId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// use crate::error::RetoroError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    id: Uuid,
    author_name: String,
    #[serde(serialize_with = "serialize_peer_id", deserialize_with = "deserialize_peer_id")]
    author_id: PeerId,
    content: String,
    timestamp: i64,
}

fn serialize_peer_id<S>(peer_id: &PeerId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let bytes = peer_id.to_bytes();
    serializer.serialize_bytes(&bytes)
}

fn deserialize_peer_id<'de, D>(deserializer: D) -> Result<PeerId, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;

    let bytes = serde::Deserialize::deserialize(deserializer)?;
    PeerId::from_bytes(bytes).map_err(D::Error::custom)
}


impl Message {
    pub fn new(author_name: String, author_id: PeerId, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            author_name,
            author_id,
            content,
            timestamp: Utc::now().timestamp_millis(),
        }
    }
    // pub fn content_as_utf8(&self) -> Result<&str, RetoroError> {
    //     Ok(std::str::from_utf8(self.content.as_slice())?)
    // }
}
