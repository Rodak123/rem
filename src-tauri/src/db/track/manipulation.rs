use anyhow::Result;
use sqlx::{Executor, Sqlite};

use crate::{db::DB, structs::audio::track::Track};

/// Insert, or refresh an existing row with the same path
pub async fn upsert<'e, E>(executor: E, t: &Track) -> Result<u64>
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
    Ok(id as u64)
}

impl DB {
    pub async fn track_init(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS track (
                id          INTEGER PRIMARY KEY,
                title       TEXT    NOT NULL,
                comment     TEXT    NOT NULL DEFAULT '',
                path        TEXT    NOT NULL UNIQUE,
                duration    INTEGER NOT NULL,
                mtime       INTEGER NOT NULL,
                hash        INTEGER,            -- NULL = not hashed yet
                fingerprint INTEGER NOT NULL,
                rating      INTEGER NOT NULL DEFAULT 0 CHECK (rating BETWEEN 0 AND 10),
                is_favorite INTEGER NOT NULL DEFAULT 0 CHECK (is_favorite IN (0, 1))
            ) STRICT
            "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn track_insert(&self, track: &Track) -> Result<u64> {
        upsert(&self.pool, track).await
    }

    /// Ids are returned in the same order as the input
    pub async fn track_insert_many(&self, tracks: &[Track]) -> Result<Vec<u64>> {
        let mut tx = self.pool.begin().await?;
        let mut ids = Vec::with_capacity(tracks.len());
        for t in tracks {
            ids.push(upsert(&mut *tx, t).await?);
        }
        tx.commit().await?;
        Ok(ids)
    }

    /// Full update by `id`. Returns `false` if no such track exists
    pub async fn track_update(&self, t: &Track) -> Result<bool> {
        // TODO: Maybe add ensurance that the rating isn't bigger than 10?
        let res = sqlx::query(
            r#"
            UPDATE track SET
                title = ?, comment = ?, path = ?, duration = ?, mtime = ?,
                hash = ?, fingerprint = ?, rating = ?, is_favorite = ?
            WHERE id = ?
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
        .bind(t.id as i64)
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected() > 0)
    }

    pub async fn track_set_rating(&self, id: u64, rating: u8) -> Result<bool> {
        // TODO: Maybe add ensurance that the rating isn't bigger than 10?
        let res = sqlx::query("UPDATE track SET rating = ? WHERE id = ?")
            .bind(rating)
            .bind(id as i64)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }

    pub async fn track_set_favorite(&self, id: u64, favorite: bool) -> Result<bool> {
        let res = sqlx::query("UPDATE track SET is_favorite = ? WHERE id = ?")
            .bind(favorite)
            .bind(id as i64)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }

    /// Returns `false` if the track didn't exist.
    pub async fn track_delete(&self, id: u64) -> Result<bool> {
        let res = sqlx::query("DELETE FROM track WHERE id = ?")
            .bind(id as i64)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }

    /// Deletes many tracks atomically. Returns how many rows were removed.
    pub async fn track_delete_many(&self, ids: &[u64]) -> Result<u64> {
        let mut tx = self.pool.begin().await?;
        let mut removed = 0;
        for &id in ids {
            removed += sqlx::query("DELETE FROM track WHERE id = ?")
                .bind(id as i64)
                .execute(&mut *tx)
                .await?
                .rows_affected();
        }
        tx.commit().await?;
        Ok(removed)
    }
}
