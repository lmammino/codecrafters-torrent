use serde::{de::Visitor, Deserialize, Deserializer};
use std::{net::Ipv4Addr, ops::Deref, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Peer {
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl FromStr for Peer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ip, port) = s.split_once(':').ok_or("Invalid peer format".to_string())?;
        let ip = Ipv4Addr::from_str(ip).map_err(|_| "Invalid IP address".to_string())?;
        let port = port.parse().map_err(|_| "Invalid port".to_string())?;
        Ok(Peer { ip, port })
    }
}

#[derive(Debug, Clone)]
pub struct Peers(Vec<Peer>);

impl Deref for Peers {
    type Target = Vec<Peer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct PeersVisitor;

impl<'de> Visitor<'de> for PeersVisitor {
    type Value = Peers;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a sequence of bytes")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let v = v.to_vec();
        let peers = v
            .chunks_exact(6)
            .map(|chunk| {
                let ip = Ipv4Addr::new(chunk[0], chunk[1], chunk[2], chunk[3]);
                let port = u16::from_be_bytes([chunk[4], chunk[5]]);
                Peer { ip, port }
            })
            .collect();

        Ok(Peers(peers))
    }
}

impl<'de> Deserialize<'de> for Peers {
    fn deserialize<D>(deserializer: D) -> Result<Peers, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(PeersVisitor)
    }
}
