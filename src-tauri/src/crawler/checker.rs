use crate::{crawler::building::hash, structs::audio::track::Track};
use anyhow::Result;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::time::Instant;

/// Groups the file based on finger print. If it finds
/// conflicts than it checks fully the file (updating hash).
/// And if it still same than the file with newest mtime wins.
/// (Tbh the only thing in that moment could be different just the path)
pub fn merge_duplicates(tracks: &mut Vec<Track>) -> Result<()> {
    let now = Instant::now();
    let mut by_fp: HashMap<u64, Vec<usize>> = HashMap::with_capacity(tracks.len());
    for (i, t) in tracks.iter().enumerate() {
        by_fp.entry(t.fingerprint).or_default().push(i);
    }
    let mut remove = vec![false; tracks.len()];

    for candidates in by_fp.into_values() {
        // There will be always 1
        // If there's more well conflicts was born
        if candidates.len() <= 1 {
            continue;
        }
        eprintln!("Found duplicated finger print");

        for &i in &candidates {
            if tracks[i].hash == None {
                tracks[i].hash = Some(hash(&tracks[i].path)?);
            }
        }
        let mut by_hash: HashMap<Option<u64>, Vec<usize>> =
            HashMap::with_capacity(candidates.len());
        for &i in &candidates {
            by_hash.entry(tracks[i].hash).or_default().push(i);
        }

        for group in by_hash.into_values() {
            // Same as before Tbh that's basically all
            // what does this algorithm
            if group.len() <= 1 {
                continue;
            }
            // newest mtime wins
            let winner = *group
                .iter()
                .max_by_key(|&&i| (tracks[i].mtime, Reverse(i)))
                .unwrap();

            let mut need_fixing = if tracks[winner].path.exists() {
                false
            } else {
                true
            };
            for i in group {
                // TODO: Someone figure it out. Too lazy :P
                if need_fixing && tracks[i].path.exists() {
                    tracks[winner].path = tracks[i].path.clone();
                    // let w_track = tracks[winner].clone();
                    // let db_cl = Arc::clone(&db);
                    // db_cl.track_update(&w_track).await;
                    need_fixing = false;
                }
                if i != winner {
                    // let id = tracks[i].id;
                    // let db_cl = Arc::clone(&db);
                    // db_cl.track_delete(id).await;
                    remove[i] = true;
                }
            }
        }
    }

    // Drop duplicated
    let mut flags = remove.into_iter();
    tracks.retain(|_| !flags.next().unwrap());
    eprintln!("Handling duplicated files took {:.2?}", now.elapsed());
    Ok(())
}
