use crate::{db::DB, structs::audio::track::Track};
use std::sync::Arc;
use tauri::{Result, State};

#[tauri::command]
pub async fn search(query: String, db: State<'_, Arc<DB>>) -> Result<Vec<Track>> {
    let tracks = db.tracks().search(&query).await?;
    Ok(tracks)
}

#[tauri::command]
pub async fn get_all_tracks(db: State<'_, Arc<DB>>) -> Result<Vec<Track>> {
    let tracks = db.tracks().all().await?;
    Ok(tracks)
}
