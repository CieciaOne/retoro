use libp2p::PeerId;

use super::{channel::Channel, common::Target};

#[derive(Debug, Clone)]
pub enum Command {
    Ping(PeerId),
    SendMessage(MessageCommand),
    JoinChannel(Channel),
    LeaveChannel(Channel),
    AddFriend(PeerId),
    RemoveFriend(PeerId),
    Shutdown,
}

#[derive(Debug, Clone)]
pub struct MessageCommand {
    pub message: String,
    pub target: Target,
}
