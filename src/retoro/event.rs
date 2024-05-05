use super::channel::Channel;
use super::common::Source;
use super::error::Error;

use libp2p::PeerId;

#[derive(Clone)]
pub enum Event {
    DiscoveredNode(PeerId),
    ReceivedMessage(MessageEvent),
    JoinedChannel(Channel),
    LeftChannel(Channel),
    AddedFriend(PeerId),
    RemoveedFriend(PeerId),
    Error(Error),
}

#[derive(Clone)]
pub struct MessageEvent {
    pub message: String,
    pub source: Source,
}
