use super::VERSION;
use rand::{distributions::Alphanumeric, Rng};

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
