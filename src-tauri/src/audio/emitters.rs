use crate::structs::audio::realtime::AudioRealTimeInfo;
use tauri::{AppHandle, Emitter};

impl AudioRealTimeInfo {
    pub fn send_realtime_data(&self, app: &AppHandle) -> tauri::Result<()> {
        app.emit("audio-realtime", self.clone())
    }
}
