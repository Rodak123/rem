use crate::{
    crawler::{building::build_track, collecting::collect_tracks, report::CrawlReport},
    structs::audio::track::Track,
    utils::path::expand_tilde,
};
use anyhow::Result;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::Instant,
};

pub mod building;
pub mod checker;
pub mod collecting;
pub mod report;

/// Blocking Walk through the directory recursively,
/// takes metadata and later makes fingerprints
/// Returned tracks are already sorted by title
///
/// This function doesn't handle duplicate fingerprints nor hashes
pub fn crawl<P: AsRef<Path>>(directory: P, known: &HashMap<PathBuf, Track>) -> Result<CrawlReport> {
    let started = Instant::now();
    let root = expand_tilde(&directory.as_ref()).into_owned();
    let cpus = std::thread::available_parallelism()?.get();

    let mut tracks = collect_tracks(&root, cpus);
    tracks.sort_unstable_by(|a, b| a.path.cmp(&b.path));
    eprintln!("Found {} tracks in {:.2?}", tracks.len(), started.elapsed());

    let pool = rayon::ThreadPoolBuilder::new().num_threads(cpus).build()?;
    let results: Vec<_> = pool.install(|| {
        tracks
            .par_iter()
            .map(|c| build_track(c, known).map_err(|e| (c.path.clone(), e)))
            .collect()
    });

    let mut tracks = Vec::with_capacity(results.len());
    let mut failed = HashMap::new();
    for r in results {
        match r {
            Ok(t) => {
                tracks.push(t);
            }
            Err((p, e)) => {
                failed.insert(p, e);
            }
        }
    }
    tracks.sort_unstable_by(|a, b| a.title.cmp(&b.title));

    eprintln!(
        "Crawled {} ({} ok, {} failed) in {:.2?}",
        root.display(),
        tracks.len(),
        failed.len(),
        started.elapsed()
    );
    Ok(CrawlReport { tracks, failed })
}
