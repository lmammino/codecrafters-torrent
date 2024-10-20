use info::Info;
use reqwest::Url;
use serde::{Deserialize, Deserializer};

mod info;
mod keys;
mod pieces;

#[derive(Debug, Deserialize, Clone)]
pub struct TorrentFile {
    /// The URL of the tracker
    #[serde(deserialize_with = "deserialize_url")]
    pub announce: Url,
    /// Details about the torrent file
    pub info: Info,
}

fn deserialize_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Url::parse(&s).map_err(serde::de::Error::custom)
}

impl TorrentFile {
    pub fn from_bytes(bytes: &[u8]) -> bendy::serde::Result<Self> {
        bendy::serde::from_bytes(bytes)
    }
}

#[cfg(test)]
mod test {
    use hex::ToHex;
    use keys::Keys;

    use super::*;

    #[test]
    fn test_deserialize_torrent_file() {
        let torrent_file = include_bytes!("../../sample.torrent");
        let torrent_file = TorrentFile::from_bytes(torrent_file).unwrap();
        assert_eq!(
            torrent_file.announce,
            Url::parse("http://bittorrent-test-tracker.codecrafters.io/announce").unwrap()
        );
        assert_eq!(
            torrent_file.info.hash().encode_hex::<String>(),
            "d69f91e6b2ae4c542468d1073a71d4ea13879a7f".to_string()
        );
        assert_eq!(torrent_file.info.name, "sample.txt");
        assert_eq!(torrent_file.info.piece_length, 32768);
        assert_eq!(torrent_file.info.pieces.len(), 3);
        let pieces_hashes = torrent_file
            .info
            .pieces
            .iter()
            .map(|x| x.encode_hex())
            .collect::<Vec<String>>();
        assert_eq!(
            pieces_hashes,
            vec![
                "e876f67a2a8886e8f36b136726c30fa29703022d",
                "6e2275e604a0766656736e81ff10b55204ad8d35",
                "f00d937a0213df1982bc8d097227ad9e909acc17"
            ]
        );

        match torrent_file.info.keys {
            Keys::SingleFile { length } => {
                assert_eq!(length, 92063);
            }
            Keys::MultipleFiles { .. } => panic!("expected single file"),
        }
    }
}
