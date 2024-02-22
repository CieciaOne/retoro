use std::str::Utf8Error;

use libp2p::identity::DecodingError;
use libp2p::noise::Error as NoiseError;
use libp2p::TransportError;
use libp2p::{gossipsub::SubscriptionError, swarm::DialError};
use openssl::error::ErrorStack;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RetoroError {
    #[error("failed subscribing to topic")]
    Network {
        #[from]
        source: SubscriptionError,
    },
    #[error("transport error")]
    Transport {
        #[from]
        source: TransportError<std::io::Error>,
    },
    #[error("failed dialing")]
    Bootnodes {
        #[from]
        source: DialError,
    },
    #[error("failed creating swarm: {source}")]
    Swarm {
        #[from]
        source: NoiseError,
    },
    #[error("IO error: {source}")]
    IO {
        #[from]
        source: std::io::Error,
    },
    #[error("failed parsing config")]
    ConfigParse {
        #[from]
        source: serde_json::Error,
    },
    #[error("failed decoding keys: {source}")]
    Decoding {
        #[from]
        source: DecodingError,
    },
    #[error("failed encoding message as utf8: {source}")]
    Utf8 {
        #[from]
        source: Utf8Error,
    },
    #[error("failed managing keys: {source}")]
    OpenSSL {
        #[from]
        source: ErrorStack,
    },
    #[error("invalid profile")]
    InvalidProfile,
}
