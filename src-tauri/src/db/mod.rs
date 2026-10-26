use crate::db::track::DBTracks;
use anyhow::Result;
use sqlx::{
    Pool, Sqlite,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};
use std::{path::Path, time::Duration};

pub struct DB {
    pub pool: Pool<Sqlite>,
}

pub mod commands;
pub mod track;
pub mod updating;

impl DB {
    /// Returns dummy struct for tracks manipulation in DB
    pub fn tracks(&self) -> DBTracks<'_> {
        DBTracks { pool: &self.pool }
    }

    /// Initialize new SQLite DB with it's corresponding parents
    ///
    /// Initialize these tables:
    ///     track
    pub async fn init(db_path: &Path) -> Result<Self> {
        if let Some(dir) = db_path.parent() {
            std::fs::create_dir_all(dir)?;
        }

        let opts = SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal)
            .foreign_keys(true)
            .busy_timeout(Duration::from_secs(5));

        let pool = SqlitePoolOptions::new()
            .max_connections(4)
            .connect_with(opts)
            .await?;
        let db = Self { pool };
        db.tracks().init().await?;

        // sqlx::migrate!().run(&pool).await?;
        Ok(db)
    }
}
