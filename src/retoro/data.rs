use super::channel::Channel;
use super::common::{deserialize_peer_id, serialize_peer_id};
use super::node::NodeRepr;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(
        serialize_with = "serialize_peer_id",
        deserialize_with = "deserialize_peer_id"
    )]
    pub local_peer_id: PeerId,
    pub nodes: Vec<NodeRepr>,
    pub channels: Vec<Channel>,
}

impl Data {
    pub fn new(id: PeerId) -> Self {
        Data {
            local_peer_id: id,
            nodes: vec![],
            channels: vec![],
        }
    }
}
