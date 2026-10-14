use anyhow::Result;
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;
use std::{fs::File, thread};
use tauri::AppHandle;

use crate::audio::request::AudioRequest;
use crate::structs::audio::options::AudioOptions;
use crate::structs::audio::realtime::AudioRealTimeInfo;
use crate::structs::audio::track::Track;

/// How does the handler works:
/// "C'mon, do something...":
///     you just use `self.send` function with
///     needed Command, like setting mute, volume, ... and it will
///     do the Rest. How do I get the handler? You just add
///     into parameters of tauri command `player: State<AudioHandler>`
///     and you get the reference of global audio handler (neat right?)
///
/// "Gimme some juicy data":
///     If you need data about current song or other things,
///     Audio handler emits data through event channels.
///     Currently there are two:
///     - RealTimeAudioInfo:
///         Current data about songs, settings,...
///         Name of the event is "audio-realtime"
///     - TrackLoadedInfo:
///         Data about song that is going to be played.
///         Name of the event is "track-loaded"
pub struct AudioHandler {
    tx: Arc<mpsc::Sender<AudioRequest>>,
    /// This is std::sync::Mutex! Meaning you must
    /// lock it and than check if it's Ok or Err.
    pub realtime_info: Arc<Mutex<AudioRealTimeInfo>>,
    pub opts: Arc<Mutex<AudioOptions>>,
}
impl AudioHandler {
    pub fn new(app: AppHandle) -> tauri::Result<Self> {
        let (tx, rx) = mpsc::channel::<AudioRequest>();
        let handler = Self {
            tx: Arc::new(tx),
            realtime_info: Arc::new(Mutex::new(AudioRealTimeInfo::new())),
            opts: Arc::new(Mutex::new(AudioOptions::new())),
        };

        let state = Arc::clone(&handler.realtime_info);
        let opts = Arc::clone(&handler.opts);
        // I guess we could use Tokio task for this, but at the same time
        // this process have tendencies to use reading files and some
        // other blocking operations (If I'm not mistake, the audio
        // backend uses some blocking operations too)
        // If we use Tokio, there would be too many extra wrappers with
        // negligible gain
        thread::Builder::new()
            .name("audio-handler".into())
            .spawn(move || {
                let device = rodio::DeviceSinkBuilder::open_default_sink()?;
                let mut player = rodio::Player::connect_new(device.mixer());
                loop {
                    match rx.recv_timeout(Duration::from_millis(250)) {
                        Ok(cmd) => match cmd {
                            AudioRequest::Load(path) => {
                                player = rodio::Player::connect_new(device.mixer());
                                if let Ok(opts) = opts.lock() {
                                    player.set_volume(opts.volume);
                                }
                                player.pause();

                                match File::open(&path).map_err(|e| e.to_string()).and_then(|f| {
                                    rodio::Decoder::try_from(f).map_err(|e| e.to_string())
                                }) {
                                    Ok(source) => {
                                        player.append(source);
                                        // This can crash just because put wrong path
                                        // App can work still
                                        // TODO: Add extra event that emits error messages
                                        let _ = Track::send_track_data(path, &app);
                                    }
                                    Err(e) => eprintln!("audio: load failed: {e}"),
                                }
                            }
                            AudioRequest::Toggle => {
                                if player.is_paused() {
                                    player.play();
                                } else {
                                    player.pause();
                                }
                            }
                            AudioRequest::Seek(dur) => {
                                // Nothing major just wrong position,
                                // still can work afterwards
                                let _ = player.try_seek(dur);
                            }
                            AudioRequest::SetVolume(v) => {
                                if let Ok(mut opts) = opts.lock() {
                                    opts.volume = v;
                                    opts.muted = false;
                                }
                                player.set_volume(v);
                            }
                            AudioRequest::Mute => {
                                if let Ok(mut opts) = opts.lock() {
                                    opts.muted = !opts.muted;
                                    player.set_volume(if opts.muted { 0.0 } else { opts.volume });
                                }
                            }
                        },
                        Err(mpsc::RecvTimeoutError::Timeout) => {}
                        Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    }
                    if let Ok(mut state) = state.lock() {
                        state.position = if player.empty() {
                            0
                        } else {
                            player.get_pos().as_secs()
                        };
                        state.finished = player.empty();
                        state.send_realtime_data(&app)?;
                    }
                }
                Ok::<(), anyhow::Error>(())
            })?;
        Ok(handler)
    }

    pub fn send(&self, cmd: AudioRequest) -> Result<()> {
        Ok(self.tx.send(cmd)?)
    }
}
