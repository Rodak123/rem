use crate::{db::DB, structs::audio::track::Track};
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn get_all_tracks(db: State<'_, Arc<DB>>) -> tauri::Result<Vec<Track>> {
    let tracks = db.track_all().await?;
    Ok(tracks)
}
