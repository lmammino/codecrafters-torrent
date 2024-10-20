use bendy::value::Value;
use client::handshake::Handshake;
use client::Client;
use hex::ToHex;
use json::bencode_to_json;
use std::env;
use std::fs;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use torrent_file::TorrentFile;
use tracker::get_tracker_info;
use tracker::peer::Peer;
use tracker::progress::Progress;
pub mod client;
pub mod json;
pub mod torrent_file;
pub mod tracker;

#[tokio::main]
// Usage: your_bittorrent.sh decode "<encoded_value>"
async fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    match command.as_str() {
        "decode" => {
            let encoded_value = &args[2];
            let bencoded_value: Value = bendy::serde::from_bytes(encoded_value.as_bytes())
                .expect("Failed to decode bencoded string");
            let json_value = bencode_to_json(&bencoded_value);

            println!("{}", json_value);
        }
        "info" => {
            let content = fs::read(&args[2]).expect("file not found");
            let torrent: TorrentFile = TorrentFile::from_bytes(&content).unwrap();
            println!("Tracker URL: {}", torrent.announce);
            println!("Length: {}", torrent.info.keys.length());
            println!("Info Hash: {}", torrent.info.hash().encode_hex::<String>());
            println!("Piece Length: {}", torrent.info.piece_length);
            let pieces_hash = torrent.info.pieces.iter();
            println!("Pieces Hash:");
            for piece_hash in pieces_hash {
                println!("{}", piece_hash.encode_hex::<String>());
            }
            println!("Info: {:?}", torrent.info);
        }
        "peers" => {
            let client = Client::default();

            let content = fs::read(&args[2]).expect("file not found");
            let torrent: TorrentFile = TorrentFile::from_bytes(&content).unwrap();
            let progress = Progress::not_started(torrent.info.keys.length());
            let tracker_info = get_tracker_info(&client, &torrent, &progress)
                .await
                .unwrap();
            println!("Interval: {}", tracker_info.interval);
            println!("Peers:");
            for peer in tracker_info.peers.iter() {
                println!("{}:{}", peer.ip, peer.port);
            }
        }
        "handshake" => {
            let client = Client::default();

            let content = fs::read(&args[2]).expect("file not found");
            let peer: Peer = args[3].parse().expect("failed to parse peer");

            let torrent: TorrentFile = TorrentFile::from_bytes(&content).unwrap();
            let mut stream = client
                .connect(peer.clone())
                .await
                .unwrap_or_else(|_| panic!("Failed to connect to peer {:?}", peer));
            println!("Connected to peer: {:?}", peer);
            let handshake = Handshake::for_client_and_torrent(&client, &torrent);
            println!("Sending handshake: {:?}", handshake);
            stream
                .write_all(&handshake.to_bytes())
                .await
                .unwrap_or_else(|_| panic!("Failed to send handshake to peer {:?}", peer));
            let mut response: Vec<u8> = vec![0; 68];
            stream.read_exact(&mut response).await.unwrap_or_else(|_| {
                panic!("Failed to read handshake response from peer {:?}", peer)
            });
            let response_handshake = Handshake::from_bytes(&response);
            println!(
                "Peer ID: {}",
                response_handshake.peer_id.encode_hex::<String>()
            );
        }
        _ => {
            println!("unknown command: {}", args[1])
        }
    };
}
