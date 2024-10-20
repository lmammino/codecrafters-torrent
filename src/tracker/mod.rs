use crate::{client::Client, torrent_file::TorrentFile};
use info::Info;
use progress::Progress;
use std::borrow::Cow;
mod info;
mod peer;
pub mod progress;

pub async fn get_tracker_info(
    client: &Client,
    torrent: &TorrentFile,
    progress: &Progress,
) -> Result<Info, anyhow::Error> {
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
        .append_pair("peer_id", &client.peer_id)
        .append_pair("port", client.port.to_string().as_str())
        .append_pair("uploaded", progress.uploaded.to_string().as_str())
        .append_pair("downloaded", progress.downloaded.to_string().as_str())
        .append_pair("left", progress.left.to_string().as_str())
        .append_pair("compact", "1")
        .finish();
    let raw_response = reqwest::get(url.to_string()).await?.bytes().await?;
    let response: Info = serde_bencode::from_bytes(&raw_response)?;
    Ok(response)
}
