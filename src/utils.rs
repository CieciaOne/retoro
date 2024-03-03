use libp2p::PeerId;

pub fn serialize_peer_id<S>(peer_id: &PeerId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let bytes = peer_id.to_bytes();
    serializer.serialize_bytes(&bytes)
}

pub fn deserialize_peer_id<'de, D>(deserializer: D) -> Result<PeerId, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;

    let bytes = serde::Deserialize::deserialize(deserializer)?;
    PeerId::from_bytes(bytes).map_err(D::Error::custom)
}
