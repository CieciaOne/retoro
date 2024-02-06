use libp2p::noise::Error as NoiseError;
use libp2p::TransportError;
use libp2p::{gossipsub::SubscriptionError, swarm::DialError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RetoroError {
    #[error("invalid rdo_lookahead_frames")]
    Network {
        #[from]
        source: SubscriptionError,
    },
    #[error("transport error")]
    Transport {
        #[from]
        source: TransportError<std::io::Error>,
    },
    #[error("invalid rdo_lookahead_frames")]
    Bootnodes {
        #[from]
        source: DialError,
    },
    #[error("failed creating swarm")]
    Swarm {
        #[from]
        source: NoiseError,
    },
    #[error("failed loading config")]
    ConfigLoad {
        #[from]
        source: std::io::Error,
    },
    #[error("failed parsing config")]
    ConfigParse {
        #[from]
        source: serde_json::Error,
    },
}

// impl From<TransportError<std::io::Error>> for RetoroError {
//     fn from(error: TransportError<std::io::Error>) -> Self {
//         RetoroError::Transport(error.to_string())
//     }
// }
