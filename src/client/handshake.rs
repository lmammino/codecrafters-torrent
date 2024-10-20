use super::Client;
use crate::torrent_file::TorrentFile;

pub const HANDSHAKE_SIZE: usize = 68;
pub const PROTOCOL_ID: [u8; 19] = *b"BitTorrent protocol";

#[derive(Debug, Clone)]
pub struct Handshake {
    pub info_hash: Vec<u8>,
    pub peer_id: Vec<u8>,
}

impl Handshake {
    pub fn new(info_hash: Vec<u8>, peer_id: Vec<u8>) -> Self {
        Handshake { info_hash, peer_id }
    }

    pub fn for_client_and_torrent(client: &Client, torrent: &TorrentFile) -> Self {
        Handshake {
            info_hash: torrent.info.hash(),
            peer_id: client.peer_id.as_bytes().to_vec(),
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        self.into()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        bytes.into()
    }
}

impl From<Handshake> for Vec<u8> {
    fn from(val: Handshake) -> Self {
        let mut msg = Vec::with_capacity(HANDSHAKE_SIZE);
        msg.push(PROTOCOL_ID.len() as u8);
        msg.extend_from_slice(&PROTOCOL_ID);
        msg.extend_from_slice(&[0; 8]);
        msg.extend_from_slice(&val.info_hash);
        msg.extend_from_slice(&val.peer_id);
        msg
    }
}

impl From<&[u8]> for Handshake {
    // FIXME: this can panic if the input is not the right size!
    fn from(val: &[u8]) -> Self {
        Handshake {
            info_hash: val[28..48].to_vec(),
            peer_id: val[48..68].to_vec(),
        }
    }
}
