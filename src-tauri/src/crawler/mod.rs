use crate::{
    crawler::{building::build_track, checker::sync_tracks, collecting::collect_tracks},
    db::DB,
    structs::audio::track::Track,
    utils::path::expand_tilde,
};
use anyhow::Result;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};
use tokio::task::JoinHandle;

pub mod building;
pub mod checker;
pub mod collecting;

pub struct CrawlReport {
    pub tracks: Vec<Track>,
    pub failed: Vec<(PathBuf, anyhow::Error)>,
}
impl CrawlReport {
    pub fn merge(&mut self, other: CrawlReport) {
        self.tracks.extend(other.tracks);
        self.failed.extend(other.failed);
    }
    pub fn empty() -> Self {
        Self {
            failed: vec![],
            tracks: vec![],
        }
    }
}

/// Walks through the directory recursively,
/// takes metadata and later makes fingerprints
/// Returned tracks are already sorted by title
fn crawl<P: AsRef<Path>>(directory: P, known: &HashMap<PathBuf, Track>) -> Result<CrawlReport> {
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
    let mut failed = Vec::new();
    for r in results {
        match r {
            Ok(t) => tracks.push(t),
            Err(e) => failed.push(e),
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
/// Crawls through directories, fixes extra files and conflicts and
/// insert into DB
/// Rn the checker is really bad
pub fn db_update(directories: Vec<PathBuf>, db: Arc<DB>) {
    tauri::async_runtime::spawn(async move {
        let known: Arc<HashMap<PathBuf, Track>> = match db.track_all().await {
            Ok(v) => Arc::new(v.into_iter().map(|t| (t.path.clone(), t)).collect()),
            Err(e) => {
                eprintln!("db: loading known tracks failed: {e}");
                Arc::default()
            }
        };

        let mut tasks = Vec::with_capacity(directories.len());
        for dir in directories {
            let known = Arc::clone(&known);
            let task: JoinHandle<Result<CrawlReport>> = tokio::task::spawn_blocking(move || {
                let crawled = crawl(dir, &known)?;
                for err in &crawled.failed {
                    eprintln!("{} | {}", err.0.display(), err.1);
                }
                Ok(crawled)
            });
            tasks.push(task);
        }

        let mut report = CrawlReport::empty();
        for task in tasks {
            if let Ok(Ok(r)) = task.await {
                report.merge(r);
            }
        }
        sync_tracks(report, Arc::clone(&known), Arc::clone(&db)).await?;
        Ok::<(), anyhow::Error>(())
    });
}
