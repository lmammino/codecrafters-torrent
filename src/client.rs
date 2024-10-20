use rand::{distributions::Alphanumeric, Rng};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Client {
    pub peer_id: String,
    pub port: u16,
}

impl Client {
    pub fn new(peer_id: impl AsRef<str>, port: u16) -> Self {
        Client {
            peer_id: peer_id.as_ref()[0..20].to_string(),
            port,
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Client::new(PeerId::default(), 6881)
    }
}

#[derive(Debug, Clone)]
pub struct PeerId(String);
impl Default for PeerId {
    fn default() -> Self {
        let version_parts = VERSION.split('.').collect::<Vec<&str>>();
        let mut peer_id = "-TH".to_string();
        for part in version_parts {
            peer_id.push_str(part);
        }
        peer_id.push('-');
        let rand_peer_id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20 - peer_id.len())
            .map(char::from)
            .collect();
        peer_id.push_str(&rand_peer_id);

        PeerId(peer_id)
    }
}

impl AsRef<str> for PeerId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
