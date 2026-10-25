use crate::{
    audio::{handler::AudioHandler, request::AudioRequest},
    db::DB,
    structs::audio::{options::AudioOptions, track::Track},
};
use lofty::{file::TaggedFileExt, probe::Probe};
use std::{path::PathBuf, sync::Arc};
use tauri::State;

#[tauri::command]
pub fn get_options(player: State<AudioHandler>) -> Result<AudioOptions, String> {
    let opts = player.opts.lock().map_err(|e| e.to_string())?;
    Ok(opts.clone())
}

#[tauri::command]
pub fn get_raw_img(path: PathBuf) -> Result<Vec<u8>, String> {
    raw_img(path).map_err(|e| e.to_string())
}
fn raw_img(path: PathBuf) -> anyhow::Result<Vec<u8>> {
    let tagged = Probe::open(&path)?.read()?;
    let tag = tagged.primary_tag().or_else(|| tagged.first_tag());
    let mut data = vec![];
    if let Some(tag) = tag {
        for p in tag.pictures() {
            if p.data().is_empty() {
                continue;
            }
            data = p.data().to_vec();
        }
    }
    Ok(data)
}

#[tauri::command]
pub async fn load_track(
    path: PathBuf,
    player: State<'_, AudioHandler>,
    db: State<'_, Arc<DB>>,
) -> tauri::Result<Option<Track>> {
    player.send(AudioRequest::Load(path.clone()))?;
    Ok(db.tracks().by_path(&path).await?)
}
