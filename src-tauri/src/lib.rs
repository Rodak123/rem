use crate::audio::{commands::getters::*, commands::setters::*, handler::AudioHandler};
use tauri::Manager;

pub mod audio;
pub mod crawler;
pub mod db;
pub mod structs;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handler = AudioHandler::new(app.handle().clone())?;
            app.manage(handler);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            load_track,
            set_volume,
            toggle,
            set_seek,
            mute,
            set_mute,
            get_options
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
