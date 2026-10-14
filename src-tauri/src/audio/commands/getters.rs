use tauri::State;

use crate::{audio::handler::AudioHandler, structs::audio::options::AudioOptions};

#[tauri::command]
pub fn get_options(player: State<AudioHandler>) -> Result<AudioOptions, String> {
    let opts = player.opts.lock().map_err(|e| e.to_string())?;
    Ok(opts.clone())
}
