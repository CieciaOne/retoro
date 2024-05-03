use libp2p::PeerId;

use super::message::Message;

struct NodeRepr {
    name: String,
    peer_id: PeerId,
}

pub struct Channel {
    name: String,
    password: Option<String>,
    nodes: Vec<NodeRepr>,
    messages: Vec<Message>,
}
