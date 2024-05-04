use chrono::Utc;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::utils::{deserialize_peer_id, serialize_peer_id};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    author_name: String,
    #[serde(
        serialize_with = "serialize_peer_id",
        deserialize_with = "deserialize_peer_id"
    )]
    author_id: PeerId,
    content: String,
    timestamp: i64,
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
    // pub fn content_as_utf8(&self) -> Result<&str, Error> {
    //     Ok(std::str::from_utf8(self.content.as_slice())?)
    // }
}
