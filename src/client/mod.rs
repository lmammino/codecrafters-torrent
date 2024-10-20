use crate::tracker::peer::Peer;
use peer_id::PeerId;
use tokio::{io, net::TcpStream};
pub mod handshake;
pub mod peer_id;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

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

    pub async fn connect(&self, peer: Peer) -> io::Result<TcpStream> {
        TcpStream::connect((peer.ip, peer.port)).await
    }
}

impl Default for Client {
    fn default() -> Self {
        Client::new(PeerId::default(), 6881)
    }
}
