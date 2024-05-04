use super::config::Config;
use super::error::Error;
use super::message::Message;
use super::utils::{deserialize_peer_id, serialize_peer_id};
use chrono::Utc;
use futures::stream::StreamExt;
use libp2p::identity::Keypair;
use libp2p::{dcutr, PeerId};
use libp2p::{
    gossipsub, mdns, noise, relay, swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, yamux,
};
use libp2p::{identify, ping, Swarm};
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::select;

pub const MAIN_NET: &str = "main";

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
    // profile: Data,
}

impl Node {
    pub fn new() -> Result<Self, Error> {
        let config = Config::default();
        let swarm = Node::swarm(&config)?;
        Ok(Self { swarm, config })
    }

    pub fn with_config(config: Config) -> Result<Self, Error> {
        let swarm = Node::swarm(&config)?;
        Ok(Self { swarm, config })
    }

    pub fn keypair(&self) -> Keypair {
        self.config.keypair()
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

    pub fn send_message(&mut self, content: String, target: String) -> Result<(), Error> {
        let name = self.config.name();
        let pk = self.config.keypair().public();
        let topic = gossipsub::IdentTopic::new(target);
        let message = Message::new(name, pk.to_peer_id(), content);
        let bytes = bincode::serialize(&message).unwrap();

        self.swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic, bytes)
            .map_err(|e| Error::Swarm(format!("Failed to send message: {e}")))?;
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

    pub async fn run(&mut self) -> Result<(), Error> {
        self.dial_known_nodes()?;
        self.start_listening()?;
        self.connect(MAIN_NET.to_string())?;

        loop {
            select! {
                // Ok(Some(line)) = stdin.next_line() => {
                //     let name = self.profile.name();
                //     let pk = self.profile.keypair()?.public();
                //     let message = Message::new(name,pk.to_peer_id(),line);
                //     let bytes = bincode::serialize(&message).unwrap();
                //     if let Err(e) = self.swarm
                //         .behaviour_mut().gossipsub
                //         .publish(topic.clone(), bytes) {
                //         error!("Publish error: {e:?}");
                //     }
                // }
                event = self.swarm.select_next_some() => match event {
                    SwarmEvent::Behaviour(RetoroBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, _multiaddr) in list {
                            debug!("mDNS discovered a new peer: {peer_id}");
                            self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        }
                    },
                    SwarmEvent::Behaviour(RetoroBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                        for (peer_id, _multiaddr) in list {
                            debug!("mDNS discover peer has expired: {peer_id}");
                            self.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                        }
                    },
                    SwarmEvent::Behaviour(RetoroBehaviourEvent::Identify(identify::Event::Received{
                            info: identify::Info { observed_addr, .. },
                            ..
                        })) => {
                            debug!("Observed new peer {}",observed_addr.clone());
                            self.swarm.add_external_address(observed_addr.clone());

                        },
                    SwarmEvent::Behaviour(RetoroBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message_id: _,
                        message,
                    })) => debug!(
                            "Message from {peer_id}: {}",
                            String::from_utf8_lossy(&message.data),
                        ),
                    SwarmEvent::NewListenAddr { address, .. } => {
                        debug!("Local node is listening on {address}");
                    }
                    _ => {}
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
