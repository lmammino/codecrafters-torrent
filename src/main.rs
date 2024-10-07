use std::env;

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
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
