use anyhow::Result;
use sqlx::SqlitePool;

pub mod deleting;
pub mod inserting;
pub mod reading;
pub mod updating;

pub struct DBTracks<'a> {
    pub pool: &'a SqlitePool,
}

impl DBTracks<'_> {
    pub async fn init(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS track (
                id          INTEGER PRIMARY KEY,
                title       TEXT    NOT NULL,
                comment     TEXT    NOT NULL DEFAULT '',
                path        TEXT    NOT NULL UNIQUE,
                duration    INTEGER NOT NULL,
                mtime       INTEGER NOT NULL,
                hash        INTEGER,
                fingerprint INTEGER NOT NULL,
                rating      INTEGER NOT NULL DEFAULT 0 CHECK (rating BETWEEN 0 AND 10),
                is_favorite INTEGER NOT NULL DEFAULT 0 CHECK (is_favorite IN (0, 1))
            ) STRICT
            "#,
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
