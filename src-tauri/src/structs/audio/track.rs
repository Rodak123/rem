use serde::Serialize;
use sqlx::prelude::FromRow;
use std::path::PathBuf;

#[derive(Clone, Serialize, Default, Debug, PartialEq, Eq)]
pub struct Track {
    // Theoretically this pretty smart because on overflow you got also
    // now the negative 8bytes
    pub id: i64,
    pub title: String,
    pub comment: String,
    pub path: PathBuf,
    /// In seconds
    pub duration: u64,
    /// Modified time
    #[serde(skip)]
    pub mtime: i64,

    #[serde(skip)]
    pub hash: Option<u64>,
    /// Basically hash, but cheap (uses only Head, Content and Tail)
    #[serde(skip)]
    pub fingerprint: u64,
    /// 0 - 10 (I hope)
    pub rating: u8,
    pub is_favorite: bool,
}

/// Alternative of Track that is actually saved (SQLite doesn't know PathBuf or u64)
#[derive(FromRow)]
pub struct TrackRow {
    pub id: i64,
    pub title: String,
    pub comment: String,
    pub path: String,
    pub duration: i64,
    pub mtime: i64,
    pub hash: Option<i64>,
    pub fingerprint: i64,
    pub rating: u8,
    pub is_favorite: bool,
}

impl From<TrackRow> for Track {
    fn from(r: TrackRow) -> Self {
        Track {
            id: r.id,
            title: r.title,
            comment: r.comment,
            path: PathBuf::from(r.path),
            duration: r.duration as u64,
            mtime: r.mtime,
            hash: r.hash.and_then(|h| Some(h as u64)),
            fingerprint: r.fingerprint as u64,
            rating: r.rating,
            is_favorite: r.is_favorite,
        }
    }
}
