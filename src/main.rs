use serde::{Deserialize, Serialize};
use serde_bencode::value::Value;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct TorrentFile {
    pub announce: String,
    pub info: HashMap<String, Value>,
}

impl TorrentFile {
    fn length(&self) -> Option<u64> {
        match self.info.get("length") {
            Some(Value::Int(length)) => Some(*length as u64),
            _ => None,
        }
    }

    fn info_hash(&self) -> Vec<u8> {
        let info = serde_bencode::to_bytes(&self.info).unwrap();
        let mut hasher = Sha1::new();
        hasher.update(&info);
        let hash = hasher.finalize();
        hash.to_vec()
    }

    fn piece_length(&self) -> Option<u64> {
        match self.info.get("piece length") {
            Some(Value::Int(piece_length)) => Some(*piece_length as u64),
            _ => None,
        }
    }

    fn pieces_hash(&self) -> Option<Vec<Vec<u8>>> {
        let pieces = self.info.get("pieces").unwrap();
        match pieces {
            Value::Bytes(pieces_bytes) => {
                let mut pieces_hash = Vec::new();
                let mut i = 0;
                while i < pieces_bytes.len() {
                    let piece_hash = &pieces_bytes[i..i + 20];
                    pieces_hash.push(piece_hash.to_vec());
                    i += 20;
                }
                Some(pieces_hash)
            }
            _ => None,
        }
    }
}

fn bytes_to_hex_string(bytes: &[u8]) -> String {
    let mut hex_string = String::new();
    for byte in bytes {
        hex_string.push_str(&format!("{:02x}", byte));
    }
    hex_string
}

fn bencode_to_json(bencode: &serde_bencode::value::Value) -> serde_json::Value {
    match bencode {
        Value::Bytes(s) => serde_json::Value::String(String::from_utf8_lossy(s).to_string()),
        Value::Int(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
        Value::List(l) => {
            let mut json_list = Vec::new();
            for item in l {
                json_list.push(bencode_to_json(item));
            }
            serde_json::Value::Array(json_list)
        }
        Value::Dict(d) => {
            let mut json_dict = serde_json::Map::new();
            for (key, value) in d {
                let new_key = String::from_utf8_lossy(key).to_string();
                json_dict.insert(new_key, bencode_to_json(value));
            }
            serde_json::Value::Object(json_dict)
        }
    }
}

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    let bencode_value: Value = serde_bencode::from_str(encoded_value).unwrap();
    bencode_to_json(&bencode_value)
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value);
    } else if command == "info" {
        let content = fs::read(&args[2]).expect("file not found");
        let torrent_file: TorrentFile = serde_bencode::from_bytes(&content).unwrap();
        println!("Tracker URL: {}", torrent_file.announce);
        println!("Length: {}", torrent_file.length().unwrap());
        println!(
            "Info Hash: {}",
            bytes_to_hex_string(&torrent_file.info_hash())
        );
        println!("Piece Length: {}", torrent_file.piece_length().unwrap());
        let pieces_hash = torrent_file.pieces_hash().unwrap();
        println!("Pieces Hash:");
        for piece_hash in pieces_hash {
            println!("{}", bytes_to_hex_string(&piece_hash));
        }
    } else {
        println!("unknown command: {}", args[1])
    }
}
