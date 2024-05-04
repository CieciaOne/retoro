use serde::{Deserialize, Serialize};

use super::{message::Message, node::NodeRepr};

#[allow(unused)]
pub struct Channel {
    name: String,
    password: Option<String>,
    nodes: Vec<NodeRepr>,
    messages: Vec<Message>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelRepr {
    name: String,
    nodes: Vec<NodeRepr>,
}
