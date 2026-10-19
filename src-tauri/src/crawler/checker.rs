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
pub async fn sync_tracks(
    report: CrawlReport,
    known: Arc<HashMap<PathBuf, Track>>,
    db: Arc<DB>,
) -> Result<()> {
    let mut tracks = report.tracks;

    // DON'T update with ids from reports. Rn All of them has id = 0
    // Only known has correct ids
    eprintln!("Deduping...");
    dedupe_found_tracks(&mut tracks, Arc::clone(&db)).await?;
    eprintln!("Resolving missing track");
    resolve_missing_tracks(&mut tracks, Arc::clone(&known), Arc::clone(&db)).await?;

    let changed: Vec<_> = tracks
        .into_iter()
        .filter(|track| known.get(&track.path) != Some(track))
        .collect();
    if !changed.is_empty() {
        eprintln!("Have to insert/update");
        // It's using upsert so you can just do this
        db.track_insert_many(&changed).await?;
    } else {
        eprintln!("Nothing new");
    }

    Ok(())
}

pub async fn dedupe_found_tracks(tracks: &mut Vec<Track>, db: Arc<DB>) -> Result<()> {
    let fg = group_by_fingerprint(tracks);
    let need_hash: Vec<(usize, PathBuf)> = fg
        .values()
        .filter(|v| v.len() > 1)
        .flatten()
        .copied()
        .filter(|&i| tracks[i].hash.is_none())
        .map(|i| (i, tracks[i].path.clone()))
        .collect();
    let computed: Vec<(usize, Option<u64>)> = tokio::task::spawn_blocking(move || {
        need_hash
            .par_iter()
            .map(|i| (i.0, hash(&i.1).ok()))
            .collect()
    })
    .await?;
    for (i, h) in computed {
        tracks[i].hash = h;
    }

    let mut remove = vec![false; tracks.len()];
    let mut delete_from_db: Vec<usize> = Vec::new();

    for fg_group in fg.values().filter(|v| v.len() > 1) {
        let mut by_hash: HashMap<u64, Vec<usize>> = HashMap::new();
        for &i in fg_group {
            if let Some(h) = tracks[i].hash {
                by_hash.entry(h).or_default().push(i);
            }
        }

        for hash_group in by_hash.values().filter(|g| g.len() > 1) {
            let mut in_db: Vec<usize> = Vec::new();
            for &i in hash_group {
                if db.track_exists_by_path(&tracks[i].path).await? {
                    in_db.push(i);
                }
            }
            let keeper = in_db.first().copied().unwrap_or(hash_group[0]);

            let (title, comment, rating, is_favorite) = {
                let members: Vec<&Track> = hash_group.iter().map(|&i| &tracks[i]).collect();
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
                (title, comment, rating, is_favorite)
            };

            let t = &mut tracks[keeper];
            t.title = title;
            t.comment = comment;
            t.rating = rating;
            t.is_favorite = is_favorite;

            // Everything except the keeper leaves the list;
            // extra copies that exist in the DB also leave the DB.
            for &i in hash_group {
                if i == keeper {
                    continue;
                }
                remove[i] = true;
                if in_db.contains(&i) {
                    delete_from_db.push(i);
                }
            }
        }
    }

    for i in delete_from_db {
        // Tbh it should be impossible to do so, because of the
        // check. If it's the case this won't ever be ran.
        // Just in case it's here
        db.delete_track_by_path(&tracks[i].path).await?;
    }

    let mut i = 0;
    tracks.retain(|_| {
        let keep = !remove[i];
        i += 1;
        keep
    });
    Ok(())
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
            let need_hash: Vec<(usize, PathBuf)> = idxs
                .iter()
                .filter(|i| tracks[**i].hash.is_none())
                .map(|i| (*i, tracks[*i].path.clone()))
                .collect();
            let computed: Vec<(usize, Option<u64>)> = tokio::task::spawn_blocking(move || {
                need_hash
                    .par_iter()
                    .map(|i| (i.0, hash(&i.1).ok()))
                    .collect()
            })
            .await?;
            for (i, h) in computed {
                tracks[i].hash = h;
            }
            *solution = idxs.iter().copied().find(|&i| {
                let t = &mut tracks[i];
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
            eprintln!("Found solution! For old {}", old.1.0.path.display());
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
