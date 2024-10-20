use bendy::value::Value;
use hex::ToHex;
use json::bencode_to_json;
use std::borrow::Cow;
use std::env;
use std::fs;
use std::net::Ipv4Addr;
use torrent_file::TorrentFile;
mod json;
mod torrent_file;

struct Peer {
    ip: Ipv4Addr,
    port: u16,
}

struct TrackerInfo {
    interval: u64,
    peers: Vec<Peer>,
}

async fn get_tracker_info(torrent: &TorrentFile) -> Result<TrackerInfo, anyhow::Error> {
    let info_hash = torrent.info.hash();
    let mut url = torrent.announce.clone();

    let url = url
        .query_pairs_mut()
        // hack from - https://stackoverflow.com/a/58027268/495177
        .encoding_override(Some(&|input| {
            if input == "{{info_hash}}" {
                Cow::Owned(info_hash.clone())
            } else {
                Cow::Borrowed(input.as_bytes())
            }
        }))
        .append_pair("info_hash", "{{info_hash}}")
        .append_pair("peer_id", "rustytorrent__v0.0.1")
        .append_pair("port", "6881")
        .append_pair("uploaded", "0")
        .append_pair("downloaded", "0")
        .append_pair("left", &torrent.info.keys.length().to_string())
        .append_pair("compact", "1")
        .finish();
    println!("{}", url);
    let raw_response = reqwest::get(url.to_string()).await?.bytes().await?;
    let response: Value = bendy::serde::from_bytes(&raw_response)?;
    println!("{:?}", response);

    todo!()
}

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
            let content = fs::read(&args[2]).expect("file not found");
            let torrent_file: TorrentFile = TorrentFile::from_bytes(&content).unwrap();
            let tracker_info = get_tracker_info(&torrent_file).await.unwrap();
            println!("Interval: {}", tracker_info.interval);
            println!("Peers:");
            for peer in tracker_info.peers {
                println!("{}:{}", peer.ip, peer.port);
            }
        }
        _ => {
            println!("unknown command: {}", args[1])
        }
    };
}
