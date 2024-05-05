use super::command::{Command, MessageCommand};
use super::common::{deserialize_peer_id, serialize_peer_id, Target};
use super::config::Config;
use super::data::Data;
use super::error::Error;
use super::event::Event;
use super::message::Message;
use chrono::Utc;
use futures::stream::StreamExt;
use libp2p::identity::Keypair;
use libp2p::{dcutr, PeerId};
use libp2p::{
    gossipsub, mdns, noise, relay, swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, yamux,
};
use libp2p::{identify, ping, Swarm};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::select;
use tokio::sync::broadcast::{Receiver as EventReceiver, Sender as EventSender};
use tokio::sync::mpsc::{Receiver as CommandReceiver, Sender as CommandSender};

pub const MAIN_NET: &str = "main";
pub const CHANNEL_SIZE: usize = 1024;

// pub trait Node{
//     fn new(name: String) -> Result<impl Sized, Error>;
//     fn with_config(config:Config) -> Self;
//     fn run(&mut self) -> Result<(), Error>;
//     fn event_stream(&self) ->impl Stream<Item = Event> + Unpin;
//     fn name(&self) -> String;
//     fn keypair(&self) -> Keypair;
//     fn channels(&self) -> Vec<Channel>;
//     fn join(&mut self, channel: String) -> Result<(),Error>;
//     fn remember(&mut self, node: NodeRepr) -> Result<(),Error>;
//     fn send_message(&mut self, message: Message, target: Target) -> Result<(), Error>;
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeRepr {
    name: String,
    #[serde(
        serialize_with = "serialize_peer_id",
        deserialize_with = "deserialize_peer_id"
    )]
    peer_id: PeerId,
}

pub struct Node {
    swarm: Swarm<RetoroBehaviour>,
    config: Config,
    data: Data,
    command_sender: CommandSender<Command>,
    command_receiver: CommandReceiver<Command>,
    event_sender: EventSender<Event>,
}

impl Node {
    pub fn new() -> Result<Self, Error> {
        let config = Config::default();
        Node::with_config(config)
    }

    pub fn with_config(config: Config) -> Result<Self, Error> {
        let swarm = Node::swarm(&config)?;
        let data = Data::new(swarm.local_peer_id().to_owned());
        let (command_sender, command_receiver) = tokio::sync::mpsc::channel(CHANNEL_SIZE);
        let (event_sender, _) = tokio::sync::broadcast::channel(CHANNEL_SIZE);
        Ok(Self {
            swarm,
            config,
            data,
            command_sender,
            command_receiver,
            event_sender,
        })
    }

    pub fn keypair(&self) -> Keypair {
        self.config.keypair()
    }

    fn send_event(&self, event: Event) -> Result<(), Error> {
        self.event_sender
            .send(event)
            .map_err(|e| Error::Transmission(format!("Failed sending event: {e}")))?;
        Ok(())
    }

    pub fn commands(&self) -> CommandSender<Command> {
        self.command_sender.clone()
    }

    pub fn events(&self) -> EventReceiver<Event> {
        self.event_sender.subscribe()
    }

    pub fn id(&self) -> PeerId {
        self.data.local_peer_id
    }
    pub fn name(&self) -> String {
        self.config.name()
    }

    fn swarm(config: &Config) -> Result<Swarm<RetoroBehaviour>, Error> {
        let swarm = libp2p::SwarmBuilder::with_existing_identity(config.keypair())
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )
            .map_err(|e| Error::Swarm(format!("Failed building swarm: {e}")))?
            .with_quic()
            .with_behaviour(|key| {
                let message_id_fn = |message: &gossipsub::Message| {
                    let mut s = DefaultHasher::new();
                    let timestamp = Utc::now().timestamp_micros();
                    timestamp.hash(&mut s);
                    message.data.hash(&mut s);
                    message.topic.hash(&mut s);
                    gossipsub::MessageId::from(s.finish().to_string())
                };
                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
                    .validation_mode(gossipsub::ValidationMode::Strict)
                    .message_id_fn(message_id_fn)
                    .build()?;
                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config,
                )?;
                let relay = relay::Behaviour::new(key.public().to_peer_id(), Default::default());
                let dcutr = dcutr::Behaviour::new(key.public().to_peer_id());
                let ping = ping::Behaviour::new(ping::Config::new());
                let identify = identify::Behaviour::new(identify::Config::new(
                    "/retoro/0.0.1".to_string(),
                    key.public(),
                ));
                let mdns = mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id(),
                )?;

                Ok(RetoroBehaviour {
                    gossipsub,
                    mdns,
                    relay,
                    dcutr,
                    identify,
                    ping,
                })
            })
            .expect("Failed creating swarm")
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        Ok(swarm)
    }

    pub fn dial_known_nodes(&mut self) -> Result<(), Error> {
        self.config
            .bootnodes()
            .iter()
            .try_for_each(|node| self.swarm.dial(node.clone()))
            .map_err(|e| Error::Swarm(format!("Failed dialing known nodes: {e}")))
    }

    fn send_message(&mut self, send_message: MessageCommand) -> Result<(), Error> {
        let message = Message::new(self.name(), self.id(), send_message.message);
        let bytes = bincode::serialize(&message).unwrap();
        match send_message.target {
            Target::Direct(_peer) => todo!(),
            Target::Channel(channel) => {
                let topic = gossipsub::IdentTopic::new(channel.name);
                self.swarm
                    .behaviour_mut()
                    .gossipsub
                    .publish(topic, bytes)
                    .map_err(|e| Error::Swarm(format!("Failed to send message: {e}")))?;
            }
        }
        Ok(())
    }

    fn start_listening(&mut self) -> Result<(), Error> {
        let addrs = self.config.interfaces();
        addrs.into_iter().try_for_each(|addr| {
            self.swarm
                .listen_on(addr)
                .map(|_| ())
                .map_err(|e| Error::Swarm(format!("Failed running the swarm: {e}")))
        })?;
        Ok(())
    }

    fn connect(&mut self, topic: String) -> Result<(), Error> {
        let t = gossipsub::IdentTopic::new(topic.clone());
        self.swarm
            .behaviour_mut()
            .gossipsub
            .subscribe(&t)
            .map_err(|e| Error::Swarm(format!("Failed subscribing to topic: {e}")))?;
        debug!("Connected to: {topic}");
        Ok(())
    }

    fn process_command(&mut self, command: Command) -> Result<(), Error> {
        match command {
            Command::SendMessage(command) => self.send_message(command)?,
            Command::Ping(_) => todo!(),
            Command::JoinChannel(_) => todo!(),
            Command::LeaveChannel(_) => todo!(),
            Command::AddFriend(_) => todo!(),
            Command::RemoveFriend(_) => todo!(),
            _ => {}
        }
        Ok(())
    }

    fn process_event(&mut self, event: SwarmEvent<RetoroBehaviourEvent>) -> Result<(), Error> {
        match event {
            SwarmEvent::Behaviour(RetoroBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                for (peer_id, _multiaddr) in list {
                    debug!("mDNS discovered a new peer: {peer_id}");
                    self.swarm
                        .behaviour_mut()
                        .gossipsub
                        .add_explicit_peer(&peer_id);
                }
            }
            SwarmEvent::Behaviour(RetoroBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                for (peer_id, _multiaddr) in list {
                    debug!("mDNS discover peer has expired: {peer_id}");
                    self.swarm
                        .behaviour_mut()
                        .gossipsub
                        .remove_explicit_peer(&peer_id);
                }
            }
            SwarmEvent::Behaviour(RetoroBehaviourEvent::Identify(identify::Event::Received {
                info: identify::Info { observed_addr, .. },
                ..
            })) => {
                debug!("Observed new peer {}", observed_addr.clone());
                self.swarm.add_external_address(observed_addr.clone());
            }
            SwarmEvent::Behaviour(RetoroBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                propagation_source: peer_id,
                message_id: _,
                message,
            })) => {
                debug!(
                    "Message from {peer_id}: {}",
                    String::from_utf8_lossy(&message.data),
                )
            }
            SwarmEvent::NewListenAddr { address, .. } => {
                debug!("Local node is listening on {address}");
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        info!("Instance started");
        self.dial_known_nodes()?;
        self.start_listening()?;
        self.connect(MAIN_NET.to_string())?;
        loop {
            select! {
                command = self.command_receiver.recv() => {
                    debug!("Command issued: {command:?}");
                    match command{
                        Some(Command::Shutdown) | None => {
                            info!("Instance shutdown");
                            return Ok(());
                        }
                        Some(command) => {
                            if let Err(e) = self.process_command(command) {
                                self.send_event(Event::Error(e))?;
                            }
                        }
                    }
                },
                event = self.swarm.select_next_some() => {
                    if let Err(e) = self.process_event(event){
                        self.send_event(Event::Error(e))?;
                    }
                }
            }
        }
    }
}

#[derive(NetworkBehaviour)]
struct RetoroBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
    relay: relay::Behaviour,
    dcutr: dcutr::Behaviour,
    identify: identify::Behaviour,
    ping: ping::Behaviour,
}
