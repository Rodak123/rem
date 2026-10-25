use std::{collections::HashMap, path::PathBuf, sync::Arc, time::Instant};

use anyhow::Result;
use tokio::task::JoinHandle;

use crate::{
    crawler::{checker::sync_tracks, crawl, report::CrawlReport},
    db::DB,
    structs::audio::track::Track,
};

/// Crawls through directories, making Report about the tracks
/// and the crawling process and synchronize the DB based on these
/// tracks.
///
/// Synchronization is meant as removing extra tracks, which points to same
/// data (same file content) and resolving tracks with missing file (tries to
/// find new file based on finger print and hash)
///
/// This process runs in different Tokio task
pub fn db_update(directories: Vec<PathBuf>, db: Arc<DB>) {
    tauri::async_runtime::spawn(async move {
        let now = Instant::now();
        let known: Arc<HashMap<PathBuf, Track>> = match db.tracks().all().await {
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
        if !report.tracks.is_empty() {
            sync_tracks(report, Arc::clone(&known), Arc::clone(&db)).await?;
        }
        eprintln!("DB update ended after in {:.2?}", now.elapsed());
        Ok::<(), anyhow::Error>(())
    });
}
