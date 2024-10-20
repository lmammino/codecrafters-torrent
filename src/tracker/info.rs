use super::peer::Peers;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Info {
    pub interval: u64,
    #[serde(rename = "min interval")]
    pub min_interval: u64,
    pub peers: Peers,
    pub complete: u64,
    pub incomplete: u64,
}
