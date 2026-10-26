use crate::db::track::DBTracks;
use anyhow::Result;
use std::path::Path;

impl DBTracks<'_> {
    /// Returns `false` if the track didn't exist
    pub async fn delete(&self, id: i64) -> Result<bool> {
        let res = sqlx::query("DELETE FROM track WHERE id = ?")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }

    /// Deletes many tracks atomically
    /// Returns how many rows were removed
    pub async fn delete_many(&self, ids: &[i64]) -> Result<u64> {
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

    /// Deletes many tracks atomically
    /// Returns how many rows were removed
    pub async fn delete_many_by_path<P: AsRef<Path>>(&self, paths: &[P]) -> Result<u64> {
        let mut tx = self.pool.begin().await?;
        let mut removed = 0;
        for path in paths {
            removed += sqlx::query("DELETE FROM track WHERE path = ?")
                .bind(path.as_ref().to_string_lossy())
                .execute(&mut *tx)
                .await?
                .rows_affected();
        }
        tx.commit().await?;
        Ok(removed)
    }

    pub async fn delete_by_path<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        sqlx::query("DELETE FROM tracks WHERE path = ?")
            .bind(path.as_ref().to_string_lossy())
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
