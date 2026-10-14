use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Serialize, Default)]
pub struct Track {
    pub hash: u64,
    pub title: String,
    pub comment: String,
    /// 0 - 10 (I hope)
    pub rating: u8,
    pub is_favorite: bool,
    pub path: PathBuf,
    /// In seconds
    pub duration: u64,
}
