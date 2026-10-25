use crate::{db::track::DBTracks, structs::audio::track::Track};
use anyhow::Result;

impl DBTracks<'_> {
    /// Full update by `id`
    /// Returns `false` if no such track exists
    pub async fn update(&self, t: &Track) -> Result<bool> {
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
        .execute(self.pool)
        .await?;
        Ok(res.rows_affected() > 0)
    }

    /// update by `path`
    /// **Won't update/use your ID!**
    /// Returns `false` if no such track exists
    pub async fn update_by_path(&self, t: &Track) -> Result<bool> {
        // TODO: Maybe add ensurance that the rating isn't bigger than 10?
        let res = sqlx::query(
            r#"
            UPDATE track SET
                title = ?, comment = ?, duration = ?, mtime = ?,
                hash = ?, fingerprint = ?, rating = ?, is_favorite = ?
            WHERE path = ?
            "#,
        )
        .bind(&t.title)
        .bind(&t.comment)
        .bind(t.duration as i64)
        .bind(t.mtime)
        .bind(t.hash.and_then(|hash| Some(hash as i64)))
        .bind(t.fingerprint as i64)
        .bind(t.rating)
        .bind(t.is_favorite)
        .bind(t.path.to_string_lossy())
        .execute(self.pool)
        .await?;
        Ok(res.rows_affected() > 0)
    }

    pub async fn set_rating(&self, id: i64, rating: u8) -> Result<bool> {
        // TODO: Maybe add ensurance that the rating isn't bigger than 10?
        let res = sqlx::query("UPDATE track SET rating = ? WHERE id = ?")
            .bind(rating)
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }

    pub async fn set_favorite(&self, id: i64, favorite: bool) -> Result<bool> {
        let res = sqlx::query("UPDATE track SET is_favorite = ? WHERE id = ?")
            .bind(favorite)
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }
}
