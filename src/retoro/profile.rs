use super::channel::ChannelRepr;
use super::node::NodeRepr;
use super::utils::{deserialize_peer_id, serialize_peer_id};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};

type Name = String;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data {
    names: Vec<Name>,
    #[serde(
        serialize_with = "serialize_peer_id",
        deserialize_with = "deserialize_peer_id"
    )]
    id: PeerId,
    nodes: Vec<NodeRepr>,
    channels: Vec<ChannelRepr>,
}

impl Data {}
