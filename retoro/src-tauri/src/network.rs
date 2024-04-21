use libp2p::{gossipsub::TopicSubscriptionFilter, PeerId};
use log::debug;

use crate::{error::RetoroError, message::Message};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Network {
    name: String,
    t: NetworkType,
    participants: Vec<PeerId>,
    messages: Vec<Message>,
}

/// Type of validation mechanism used when connecting to networks.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum NetworkType {
    /// Open network is one that anyone can join freely.
    Public,
    /// Protected network is multinode network reqiring authentication.
    Protected(String), // TODO instead of string it should have some kind of validator
    /// Private network is a network between only 2 nodes to communicate directly
    Private(PeerId),
}

impl Network {
    pub fn new(name: String, t: NetworkType) -> Result<Self, RetoroError> {
        if name.is_empty() {
            return Err(RetoroError::Swarm(format!("Invalid network name: {name}")));
        }
        Ok(Self {
            name,
            t,
            participants: vec![],
            messages: vec![],
        })
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn network_type(&self) -> NetworkType {
        self.t.to_owned()
    }
}

#[derive(Clone, Default)]
pub struct NetworkSubscriptionFilter;

impl TopicSubscriptionFilter for NetworkSubscriptionFilter {
    fn can_subscribe(&mut self, topic_hash: &libp2p::gossipsub::TopicHash) -> bool {
        debug!("topic:{}", topic_hash.as_str());

        true
    }
}
