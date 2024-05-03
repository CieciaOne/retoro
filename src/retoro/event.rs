use libp2p::PeerId;

use super::{channel::Channel, message::Message};

pub enum Event {
    ReceivedMessage(Message, Source),
    JoinedChannel(Channel, PeerId), // should return propper channel and the node leaving
    LeftChannel(Channel, PeerId),   //should return something like who and which channel
    AddedAsFriend(PeerId),
}

enum Source {
    Direct(PeerId),
    Channel(Channel),
}
