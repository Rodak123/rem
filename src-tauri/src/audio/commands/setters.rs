use crate::audio::{handler::AudioHandler, request::AudioRequest};
use std::time::Duration;
use tauri::{Result, State};

#[tauri::command]
pub fn toggle(player: State<AudioHandler>) -> Result<()> {
    Ok(player.send(AudioRequest::Toggle)?)
}

#[tauri::command]
pub fn set_volume(volume: f32, player: State<AudioHandler>) -> Result<()> {
    Ok(player.send(AudioRequest::SetVolume(volume))?)
}

#[tauri::command]
pub fn set_seek(duration: u64, player: State<AudioHandler>) -> Result<()> {
    Ok(player.send(AudioRequest::Seek(Duration::from_secs(duration)))?)
}

#[tauri::command]
pub fn mute(player: State<AudioHandler>) -> Result<()> {
    Ok(player.send(AudioRequest::Mute)?)
}

#[tauri::command]
pub fn set_mute(player: State<AudioHandler>, muted: bool) -> Result<()> {
    if let Ok(opts) = player.opts.lock()
        && opts.muted != muted
    {
        player.send(AudioRequest::Mute)?;
    }
    Ok(())
}
