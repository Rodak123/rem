use anyhow::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    crawler::{CrawlReport, building::hash},
    db::DB,
    structs::audio::track::Track,
    utils::scorer::new_tracks::most_common,
};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

fn group_by_fingerprint(tracks: &[Track]) -> HashMap<u64, Vec<usize>> {
    let mut m: HashMap<u64, Vec<usize>> = HashMap::with_capacity(tracks.len());
    for (i, t) in tracks.iter().enumerate() {
        m.entry(t.fingerprint).or_default().push(i);
    }
    m
}

/// Removes old tracks from DB, which couldn't be fixed
/// or updates relation to newest tracks, which were found
/// If report found tracks with same hash, it will make
/// mutant with the most common values and merge it.
///
/// TODO: Finish up DB manipulation:
///     [ ] Updating older tracks
///     [ ] Inserting new tracks
///     [X] Deleting missing tracks
pub async fn sync_tracks(
    report: CrawlReport,
    known: Arc<HashMap<PathBuf, Track>>,
    db: Arc<DB>,
) -> Result<()> {
    let mut tracks = report.tracks;

    // DON'T update with ids from reports. Rn All of them has id = 0
    // Only known has correct ids
    let mut tracks = tokio::task::spawn_blocking(move || {
        dedupe_found_tracks(&mut tracks);
        tracks
    })
    .await?;
    resolve_missing_tracks(&mut tracks, Arc::clone(&known), Arc::clone(&db)).await?;
    Ok(())
}

pub fn dedupe_found_tracks(tracks: &mut Vec<Track>) {
    let fg = group_by_fingerprint(tracks);
    let need_hash: Vec<usize> = fg
        .values()
        .filter(|v| v.len() > 1)
        .flatten()
        .copied()
        .filter(|&i| tracks[i].hash.is_none())
        .collect();
    let computed: Vec<(usize, Option<u64>)> = need_hash
        .par_iter()
        .map(|&i| (i, hash(&tracks[i].path).ok()))
        .collect();
    for (i, h) in computed {
        tracks[i].hash = h;
    }

    let mut remove = vec![false; tracks.len()];

    for idxs in fg.values().filter(|v| v.len() > 1) {
        let mut by_hash: HashMap<u64, Vec<usize>> = HashMap::new();
        for &i in idxs {
            if let Some(h) = tracks[i].hash {
                by_hash.entry(h).or_default().push(i);
            }
        }

        for group in by_hash.values().filter(|g| g.len() > 1) {
            let first = group[0];
            let members: Vec<&Track> = group.iter().map(|&i| &tracks[i]).collect();

            let title = most_common(
                members
                    .iter()
                    .map(|t| &t.title)
                    .filter(|s| !s.trim().is_empty())
                    .cloned(),
            )
            .unwrap_or_else(|| members[0].title.clone());
            let comment = most_common(
                members
                    .iter()
                    .map(|t| &t.comment)
                    .filter(|s| !s.trim().is_empty())
                    .cloned(),
            )
            .unwrap_or_else(|| members[0].comment.clone());
            let rating = most_common(members.iter().map(|t| t.rating).filter(|&r| r > 0))
                .unwrap_or(members[0].rating);
            let is_favorite = members.iter().any(|t| t.is_favorite);
            // `members` is dead after this point, so the mutable borrow below is fine

            let t = &mut tracks[first];
            t.title = title;
            t.comment = comment;
            t.rating = rating;
            t.is_favorite = is_favorite;

            for &i in &group[1..] {
                remove[i] = true;
            }
        }
    }

    let mut i = 0;
    tracks.retain(|_| {
        let keep = !remove[i];
        i += 1;
        keep
    });
}

/// Takes old tracks pointing to path,
/// which doesn't exist and try to find
/// new solution with same fingerprint,
/// hash, score,...
async fn resolve_missing_tracks(
    tracks: &mut Vec<Track>,
    known: Arc<HashMap<PathBuf, Track>>,
    db: Arc<DB>,
) -> Result<()> {
    let fg = group_by_fingerprint(tracks);
    // The option track is solution for the removal (new file)
    let mut should_remove: HashMap<&PathBuf, (&Track, Option<usize>)> = known
        .iter()
        .filter(|k| !k.1.path.exists())
        .map(|r| (r.0, (r.1, None)))
        .collect();

    // Trying to fix the should_remove records
    for (old_t, solution) in should_remove.values_mut() {
        let Some(idxs) = fg.get(&old_t.fingerprint) else {
            continue;
        };

        if let Some(old_hash) = old_t.hash {
            *solution = idxs.iter().copied().find(|&i| {
                let t = &mut tracks[i];
                if t.hash.is_none() {
                    t.hash = hash(&t.path).ok();
                }
                t.hash == Some(old_hash)
            });
        } else {
            *solution = idxs
                .iter()
                .copied()
                .map(|i| {
                    let t = &tracks[i];
                    let score = (old_t.title == t.title) as u8
                        + (old_t.comment == t.comment) as u8
                        + (old_t.duration == t.duration) as u8
                        + (old_t.mtime == t.mtime) as u8
                        + (idxs.len() == 1) as u8;
                    (score, std::cmp::Reverse(i))
                })
                .filter(|(s, _)| *s > 0)
                .max()
                .map(|(_, std::cmp::Reverse(i))| i);
        }
    }

    // Removal
    for old in should_remove {
        if let Some(_solution) = old.1.1 {
            // Updating the relations
            // Something like update relation set old_id = new_id
            // blah blah you get it
        }
        // It's little bit aggressive. You'll loose all connections
        // with this track if you move it sync it and then return it back
        // but tbh it's really niche situation, which doesn't just happen
        // every day (I hope)
        db.track_delete(old.1.0.id).await?;
    }
    Ok(())
}
