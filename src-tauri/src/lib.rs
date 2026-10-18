use crate::{
    audio::{
        commands::{getters::*, setters::*},
        handler::AudioHandler,
    },
    crawler::db_update,
    db::{DB, commands::setters::*},
};
use std::sync::Arc;
use tauri::Manager;

pub mod audio;
pub mod crawler;
pub mod db;
pub mod structs;
pub mod utils;

pub const DB_FILE: &str = "database.db";
pub const SUPPORTED_EXTENSIONS: [&str; 14] = [
    "aac", "adpcm", "aiff", "alac", "caf", "flac", "mkv", "mp1", "mp2", "mp3", "mp4", "vorbis",
    "wav", "webm",
];

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let audio_dir = app.path().audio_dir()?;
            let app_data = app.path().app_data_dir()?;

            let db = Arc::new(tauri::async_runtime::block_on(async {
                let db = DB::init(&app_data.join(DB_FILE)).await?;
                db.track_init().await?;
                anyhow::Ok(db)
            })?);

            db_update(vec![audio_dir], Arc::clone(&db));
            app.manage(AudioHandler::new(app.handle().clone())?);
            app.manage(db);
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
            get_options,
            get_all_tracks,
            get_raw_img
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
