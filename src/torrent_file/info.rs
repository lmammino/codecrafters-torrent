use super::{keys::Keys, pieces::Pieces};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Info {
    /// If single file: a UTF-8 encoded string which is the suggested name to save the file (or directory) as.
    /// It is purely advisory.
    /// If multi file: the name of the directory in which to store all the files.
    pub name: String,
    /// If single file: a struct that contains the length of the file in bytes.
    /// If multi file: a struct that contains a list of dictionaries, one for each file.
    #[serde(flatten)]
    pub keys: Keys,
    /// the number of bytes in each piece the file is split into.
    /// For the purposes of transfer, files are split into fixed-size pieces
    /// which are all the same length except for possibly the last one which may be truncated.
    /// piece length is almost always a power of two,
    /// most commonly 2 18 = 256 K (BitTorrent prior to version 3.2 uses 2 20 = 1 M as default).
    #[serde(rename = "piece length")]
    pub piece_length: u64,
    // a string whose length is a multiple of 20. It is to be subdivided into strings of length 20,
    // each of which is the SHA1 hash of the piece at the corresponding index.
    pub pieces: Pieces,
}

impl Info {
    pub fn hash(&self) -> Vec<u8> {
        let info = bendy::serde::to_bytes(self).unwrap();
        let mut hasher = Sha1::new();
        hasher.update(&info);
        hasher.finalize().to_vec()
    }
}
