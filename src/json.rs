use bendy::value::Value as BencodeValue;
use serde_json::Value as JsonValue;

pub fn bencode_to_json(bencode: &BencodeValue) -> JsonValue {
    match bencode {
        BencodeValue::Bytes(s) => serde_json::Value::String(String::from_utf8_lossy(s).to_string()),
        BencodeValue::Integer(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
        BencodeValue::List(l) => {
            let mut json_list = Vec::new();
            for item in l {
                json_list.push(bencode_to_json(item));
            }
            serde_json::Value::Array(json_list)
        }
        BencodeValue::Dict(d) => {
            let mut json_dict = serde_json::Map::new();
            for (key, value) in d {
                let new_key = String::from_utf8_lossy(key).to_string();
                json_dict.insert(new_key, bencode_to_json(value));
            }
            serde_json::Value::Object(json_dict)
        }
    }
}
