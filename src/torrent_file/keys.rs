use serde::{Deserialize, Serialize};

/// Enum that allows to distinguish between single file torrents
/// and multi-file torrents.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Keys {
    SingleFile { length: u64 },
    MultipleFiles { files: Vec<FileInfo> },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileInfo {
    /// the length of the file in bytes
    pub length: u64,
    /// a list of UTF-8 encoded strings corresponding to subdirectory names,
    /// the last of which is the actual file name (a zero length list is an error case).
    pub path: Vec<String>,
}

impl Keys {
    pub fn length(&self) -> u64 {
        match self {
            Keys::SingleFile { length } => *length,
            Keys::MultipleFiles { files } => files.iter().map(|file| file.length).sum(),
        }
    }
}
