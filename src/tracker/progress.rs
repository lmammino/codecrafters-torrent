pub struct Progress {
    pub downloaded: u64,
    pub uploaded: u64,
    pub left: u64,
}

impl Progress {
    #[allow(dead_code)]
    pub fn new(downloaded: u64, uploaded: u64, left: u64) -> Self {
        Progress {
            downloaded,
            uploaded,
            left,
        }
    }

    pub fn not_started(left: u64) -> Self {
        Progress {
            downloaded: 0,
            uploaded: 0,
            left,
        }
    }
}
