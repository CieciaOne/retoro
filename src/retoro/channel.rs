use serde::{Deserialize, Serialize};

use super::{message::Message, node::NodeRepr};

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub name: String,
    password: Option<String>,
    nodes: Vec<NodeRepr>,
    messages: Vec<Message>,
}

impl Channel {
    pub fn new(name: String, password: Option<String>) -> Self {
        Self {
            name,
            password,
            nodes: vec![],
            messages: vec![],
        }
    }
}
