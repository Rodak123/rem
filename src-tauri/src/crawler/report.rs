use std::{collections::HashMap, path::PathBuf};

use crate::structs::audio::track::Track;

pub struct CrawlReport {
    pub tracks: Vec<Track>,
    pub failed: HashMap<PathBuf, anyhow::Error>,
}
impl CrawlReport {
    pub fn merge(&mut self, other: CrawlReport) {
        self.tracks.extend(other.tracks);
        self.failed.extend(other.failed);
    }
    pub fn empty() -> Self {
        Self {
            failed: HashMap::new(),
            tracks: vec![],
        }
    }
}
