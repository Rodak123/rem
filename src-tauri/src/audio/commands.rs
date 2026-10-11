use crate::audio::handler::AudioHandler;
use std::{path::PathBuf, time::Duration};
use tauri::State;

pub enum AudioCommand {
    // Because we have Toggle there's no need for play/pause
    // If it is needed, implementation should be fairly simple
    Toggle,
    Load(PathBuf),
    SetVolume(f32),
    Seek(Duration),
    Mute,
}

#[tauri::command]
pub fn load_track(path: PathBuf, player: State<AudioHandler>) {
    player.send(AudioCommand::Load(path));
}

#[tauri::command]
pub fn toggle(player: State<AudioHandler>) {
    player.send(AudioCommand::Toggle);
}

#[tauri::command]
pub fn set_volume(volume: f32, player: State<AudioHandler>) {
    player.send(AudioCommand::SetVolume(volume));
}

#[tauri::command]
pub fn set_seek(duration: u64, player: State<AudioHandler>) {
    player.send(AudioCommand::Seek(Duration::from_secs(duration)));
}

#[tauri::command]
pub fn mute(player: State<AudioHandler>) {
    player.send(AudioCommand::Mute);
}

#[tauri::command]
pub fn set_mute(player: State<AudioHandler>, muted: bool) {
    let info = &player.real_time.info;
    if info.muted != muted {
        player.send(AudioCommand::Mute);
    }
}
