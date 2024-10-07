use serde::{Deserialize, Serialize};
use serde_bencode::value::Value;
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
}

fn bencode_to_json(bencode: &serde_bencode::value::Value) -> serde_json::Value {
    match bencode {
        serde_bencode::value::Value::Bytes(s) => {
            serde_json::Value::String(String::from_utf8_lossy(s).to_string())
        }
        serde_bencode::value::Value::Int(i) => {
            serde_json::Value::Number(serde_json::Number::from(*i))
        }
        serde_bencode::value::Value::List(l) => {
            let mut json_list = Vec::new();
            for item in l {
                json_list.push(bencode_to_json(item));
            }
            serde_json::Value::Array(json_list)
        }
        serde_bencode::value::Value::Dict(d) => {
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
    let bencode_value: serde_bencode::value::Value =
        serde_bencode::from_str(encoded_value).unwrap();
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
    } else {
        println!("unknown command: {}", args[1])
    }
}
