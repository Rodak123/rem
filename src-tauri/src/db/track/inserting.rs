use crate::{db::track::DBTracks, structs::audio::track::Track};
use anyhow::Result;
use sqlx::{Executor, Sqlite};

/// Insert, or refresh an existing row with the same path
pub async fn upsert<'e, E>(executor: E, t: &Track) -> Result<i64>
where
    E: 'e + Executor<'e, Database = Sqlite>,
{
    let id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO track
            (title, comment, path, duration, mtime, hash, fingerprint, rating, is_favorite)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(path) DO UPDATE SET
            title       = excluded.title,
            comment     = excluded.comment,
            duration    = excluded.duration,
            mtime       = excluded.mtime,
            fingerprint = excluded.fingerprint,
            hash        = CASE WHEN excluded.fingerprint = track.fingerprint
                               THEN COALESCE(excluded.hash, track.hash)
                               ELSE excluded.hash END
        RETURNING id
        "#,
    )
    .bind(&t.title)
    .bind(&t.comment)
    .bind(t.path.to_string_lossy())
    .bind(t.duration as i64)
    .bind(t.mtime)
    .bind(t.hash.and_then(|hash| Some(hash as i64)))
    .bind(t.fingerprint as i64)
    .bind(t.rating)
    .bind(t.is_favorite)
    .fetch_one(executor)
    .await?;
    Ok(id)
}

impl DBTracks<'_> {
    /// Inserts or updates based on the path.
    /// ID of your Track isn't used. SQLite will figure it out itself.
    /// Values used: title, comment, path, duration, mtime, hash, fingerprint, rating, is_favorite
    /// Returns id of inserted/updated track
    pub async fn insert(&self, track: &Track) -> Result<i64> {
        upsert(self.pool, track).await
    }

    /// Ids are returned in the same order as the input
    pub async fn insert_many(&self, tracks: &[Track]) -> Result<Vec<i64>> {
        let mut tx = self.pool.begin().await?;
        let mut ids = Vec::with_capacity(tracks.len());
        for t in tracks {
            ids.push(upsert(&mut *tx, t).await?);
        }
        tx.commit().await?;
        Ok(ids)
    }
}
