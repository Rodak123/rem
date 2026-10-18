use ignore::{WalkBuilder, WalkState};
use std::{
    path::{Path, PathBuf},
    sync::mpsc,
    time::UNIX_EPOCH,
};

use crate::utils::audio::is_supported;

pub struct CollectedTracks {
    pub path: PathBuf,
    pub size: u64,
    pub mtime: i64,
}

/// Will look into directory recursively and make
/// shallow track collection (simple data without
/// actually reading the file)
pub fn collect_tracks(root: &Path, walk_threads: usize) -> Vec<CollectedTracks> {
    let (tx, rx) = mpsc::channel();
    WalkBuilder::new(root)
        .threads(walk_threads)
        .follow_links(false)
        .parents(false)
        .build_parallel()
        .run(|| {
            let tx = tx.clone();
            Box::new(move |res| {
                let Ok(entry) = res else {
                    return WalkState::Continue;
                };
                let Ok(md) = entry.metadata() else {
                    return WalkState::Continue;
                };
                let path = entry.path().to_path_buf();
                if !entry.file_type().is_some_and(|t| t.is_file()) || !is_supported(&path) {
                    return WalkState::Continue;
                }
                let mtime = md
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                    .map_or(0, |d| d.as_secs() as i64);
                // I know we could just also do the full build of Track,
                // but I think this is way more intuitive and less complex
                let _ = tx.send(CollectedTracks {
                    path,
                    size: md.len(),
                    mtime,
                });
                WalkState::Continue
            })
        });
    drop(tx);
    rx.into_iter().collect()
}
