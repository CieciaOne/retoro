use crate::config;
use crate::error;
use crate::message::Message;
use crate::network::Network;
use crate::network::NetworkSubscriptionFilter;
use crate::network::NetworkType;
use crate::profile::Profile;

use chrono::Utc;
use config::Config;
use error::RetoroError;
use futures::stream::StreamExt;
use libp2p::dcutr;
use libp2p::gossipsub::IdentityTransform;
use libp2p::{
    gossipsub, mdns, noise, relay, swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, yamux,
};
use libp2p::{identify, ping, Swarm};
use log::debug;
use log::error;
use log::info;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::{io, io::AsyncBufReadExt, select};

pub struct Retoro {
    swarm: Swarm<RetoroBehaviour>,
    config: Config,
    profile: Profile,
}

impl Retoro {
    pub async fn new(config: Config, profile: Profile) -> Result<Self, RetoroError> {
        let swarm = Retoro::swarm(&profile).await?;
        Ok(Self {
            swarm,
            config,
            profile,
        })
    }

    async fn swarm(profile: &Profile) -> Result<Swarm<RetoroBehaviour>, RetoroError> {
        let swarm = libp2p::SwarmBuilder::with_existing_identity(profile.keypair()?)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )
            .map_err(|e| RetoroError::Swarm(format!("Failed building swarm: {e}")))?
            .with_quic()
            .with_behaviour(|key| {
                // To content-address message, we can take the hash of message and use it as an ID.
                let message_id_fn = |message: &gossipsub::Message| {
                    let mut s = DefaultHasher::new();
                    let timestamp = Utc::now().timestamp_micros();
                    timestamp.hash(&mut s);
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
                let gossipsub = gossipsub::Behaviour::new_with_subscription_filter(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config,
                    None,
                    NetworkSubscriptionFilter,
                )?;

                let relay = relay::Behaviour::new(key.public().to_peer_id(), Default::default());

                let dcutr = dcutr::Behaviour::new(key.public().to_peer_id());

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

    pub async fn dial_known_nodes(&mut self) -> Result<(), RetoroError> {
        self.config
            .get_bootnodes()
            .iter()
            .try_for_each(|node| self.swarm.dial(node.clone()))
            .map_err(|e| RetoroError::Swarm(format!("Failed dialing known nodes: {e}")))
    }

    pub fn connect_public(&mut self, network: Network) -> Result<bool, RetoroError> {
        let topic = gossipsub::IdentTopic::new(network.name());

        info!("Connecting to {0}[{topic}]", network.name());
        match network.network_type() {
            NetworkType::Public => self
                .swarm
                .behaviour_mut()
                .gossipsub
                .subscribe(&topic)
                .map_err(|e| RetoroError::Swarm(format!("Failed subscribing to topic: {e}"))),
            NetworkType::Protected(_) => todo!(),
            NetworkType::Private(_) => todo!(),
        }
    }

    pub async fn run(&mut self) -> Result<(), RetoroError> {
        self.dial_known_nodes().await?;
        // Read full lines from stdin as temporary measure for testing
        let mut stdin = io::BufReader::new(io::stdin()).lines();

        let addrs = self.config.get_addrs();
        addrs.into_iter().try_for_each(|addr| {
            self.swarm
                .listen_on(addr)
                .map(|_| ())
                .map_err(|e| RetoroError::Swarm(format!("Failed running the swarm: {e}")))
        })?;

        let main_net = gossipsub::IdentTopic::new("main");
        let known_nets = self.profile.known_networks();

        known_nets
            .into_iter()
            .for_each(|net| match self.connect_public(net) {
                Ok(_) => {}
                Err(e) => error!("{e}"),
            });

        loop {
            select! {
                Ok(Some(line)) = stdin.next_line() => {
                    let name = self.profile.name();
                    let pk = self.profile.keypair()?.public();
                    let message = Message::new(name,pk.to_peer_id(),line);
                    let bytes = bincode::serialize(&message).unwrap();
                    if let Err(e) = self.swarm
                        .behaviour_mut().gossipsub
                        .publish(main_net.clone(), bytes) {
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

#[derive(NetworkBehaviour)]
struct RetoroBehaviour {
    gossipsub: gossipsub::Behaviour<IdentityTransform, NetworkSubscriptionFilter>,
    mdns: mdns::tokio::Behaviour,
    relay: relay::Behaviour,
    dcutr: dcutr::Behaviour,
    identify: identify::Behaviour,
    ping: ping::Behaviour,
}
