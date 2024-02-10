use crate::config;
use crate::error;

use config::Config;
use error::RetoroError;
use futures::stream::StreamExt;
use libp2p::{
    gossipsub, mdns, noise, relay, swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, yamux,
};
use libp2p::{identify, ping, Swarm};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::{io, io::AsyncBufReadExt, select};
use tracing::debug;
use tracing::error;

pub struct Retoro {
    swarm: Swarm<RetoroBehaviour>,
    config: Config,
}

impl Retoro {
    pub async fn new(config: Config) -> Result<Self, RetoroError> {
        Ok(Self {
            swarm: Retoro::swarm().await?,
            config,
        })
    }

    async fn swarm() -> Result<Swarm<RetoroBehaviour>, RetoroError> {
        let swarm = libp2p::SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_quic()
            .with_behaviour(|key| {
                // To content-address message, we can take the hash of message and use it as an ID.
                let message_id_fn = |message: &gossipsub::Message| {
                    let mut s = DefaultHasher::new();
                    message.data.hash(&mut s);
                    message.topic.hash(&mut s);
                    message.source.hash(&mut s);
                    gossipsub::MessageId::from(s.finish().to_string())
                };

                // Set a custom gossipsub configuration
                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
                    .validation_mode(gossipsub::ValidationMode::Strict)
                    .message_id_fn(message_id_fn)
                    .build()?;

                // build a gossipsub network behaviour
                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config,
                )?;

                let relay = relay::Behaviour::new(key.public().to_peer_id(), Default::default());

                let ping = ping::Behaviour::new(ping::Config::new());

                let identify = identify::Behaviour::new(identify::Config::new(
                    "/TODO/0.0.1".to_string(),
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
                    identify,
                    ping,
                })
            })
            .expect("Failed creating swarm")
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        Ok(swarm)
    }

    pub async fn setup_bootnodes(&mut self) -> Result<(), RetoroError> {
        Ok(self
            .config
            .get_bootnodes()
            .iter()
            .try_for_each(|node| self.swarm.dial(node.clone()))?)
    }
    pub async fn run(&mut self) -> Result<(), RetoroError> {
        self.setup_bootnodes().await?;
        // Read full lines from stdin
        let mut stdin = io::BufReader::new(io::stdin()).lines();
        let listen_addr_quic = self.config.get_quic_addrs();
        let listen_addr_tcp = self.config.get_tcp_addrs();
        self.swarm.listen_on(listen_addr_tcp)?;
        self.swarm.listen_on(listen_addr_quic)?;

        // Create a Gossipsub topic
        let topic = gossipsub::IdentTopic::new("test-net");
        // subscribes to our topic
        self.swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        loop {
            select! {
                Ok(Some(line)) = stdin.next_line() => {
                    if let Err(e) = self.swarm
                        .behaviour_mut().gossipsub
                        .publish(topic.clone(), line.as_bytes()) {
                        error!("Publish error: {e:?}");
                    }
                }
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

// We create a custom network behaviour that combines Gossipsub and Mdnd with Relay.
#[derive(NetworkBehaviour)]
struct RetoroBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
    relay: relay::Behaviour,
    identify: identify::Behaviour,
    ping: ping::Behaviour,
}
