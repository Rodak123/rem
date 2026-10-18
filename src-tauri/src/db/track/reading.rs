use anyhow::Result;
use std::path::Path;

use crate::{
    db::DB,
    structs::audio::track::{Track, TrackRow},
};

/// Makes query for selecting every property in track
/// Passed string is used as tail (condition, ordering, ...)
macro_rules! select_track {
    ($tail:literal) => {
        concat!(
            "SELECT id, title, comment, path, duration, mtime, hash, fingerprint, \
             rating, is_favorite FROM track ",
            $tail
        )
    };
}

/// These are only for selecting
impl DB {
    pub async fn track_get(&self, id: u64) -> Result<Option<Track>> {
        let row = sqlx::query_as::<_, TrackRow>(select_track!("WHERE id = ?"))
            .bind(id as i64)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(Track::from))
    }

    pub async fn track_get_by_path<P: AsRef<Path>>(&self, path: P) -> Result<Option<Track>> {
        let path = path.as_ref().to_string_lossy();
        let row = sqlx::query_as::<_, TrackRow>(select_track!("WHERE path = ?"))
            .bind(path)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(Track::from))
    }

    /// Sorted by title
    pub async fn track_all(&self) -> Result<Vec<Track>> {
        let rows =
            sqlx::query_as::<_, TrackRow>(select_track!("ORDER BY title COLLATE NOCASE, id"))
                .fetch_all(&self.pool)
                .await?;
        Ok(rows.into_iter().map(Track::from).collect())
    }

    /// Returns all tracks, where favorite = 1
    pub async fn track_favorites(&self) -> Result<Vec<Track>> {
        let rows = sqlx::query_as::<_, TrackRow>(select_track!(
            "WHERE is_favorite = 1 ORDER BY title COLLATE NOCASE, id"
        ))
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(Track::from).collect())
    }

    /// Substring search over title and comment, case-insensitive for ASCII.
    pub async fn track_search(&self, query: &str) -> Result<Vec<Track>> {
        let mut pattern = String::with_capacity(query.len() + 2);
        pattern.push('%');
        for c in query.chars() {
            // User put escape character (we must add \)
            if matches!(c, '%' | '_' | '\\') {
                pattern.push('\\');
            }
            pattern.push(c);
        }
        pattern.push('%');

        let rows = sqlx::query_as::<_, TrackRow>(select_track!(
            r"WHERE title LIKE ?1 ESCAPE '\' OR comment LIKE ?1 ESCAPE '\'
              ORDER BY title COLLATE NOCASE, id"
        ))
        .bind(pattern)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(Track::from).collect())
    }
}
