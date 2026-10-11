use lofty::file::AudioFile;
use lofty::probe::Probe;
use tauri::{AppHandle, Emitter};

use crate::audio::commands::AudioCommand;
use std::sync::mpsc;
use std::time::Duration;
use std::{fs::File, thread};

#[derive(Clone, serde::Serialize)]
pub struct Progress {
    pub position_ms: u64,
    pub finished: bool,
}
impl Progress {
    pub fn new() -> Self {
        Self {
            finished: false,
            position_ms: 0,
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct AudioInfo {
    pub volume: f32,
    pub muted: bool,
}
impl AudioInfo {
    pub fn new() -> Self {
        Self {
            muted: false,
            volume: 1.0,
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct RealTimeAudioInfo {
    pub progress: Progress,
    pub info: AudioInfo,
}
impl RealTimeAudioInfo {
    pub fn new() -> Self {
        Self {
            progress: Progress::new(),
            info: AudioInfo::new(),
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct TrackLoadedAudioInfo {
    pub duration_ms: u64,
}
impl TrackLoadedAudioInfo {
    pub fn new() -> Self {
        Self { duration_ms: 0 }
    }
}

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
    tx: mpsc::Sender<AudioCommand>,
    pub real_time: RealTimeAudioInfo,
    pub track_loaded: TrackLoadedAudioInfo,
}
impl AudioHandler {
    pub fn new(app: AppHandle) -> Self {
        let (tx, rx) = mpsc::channel::<AudioCommand>();
        let mut handler = Self {
            tx,
            real_time: RealTimeAudioInfo::new(),
            track_loaded: TrackLoadedAudioInfo::new(),
        };

        // I guess we could use Tokio task for this, but at the same time
        // this process have tendencies to use reading files and some
        // other blocking operations (If I'm not mistake, the audio
        // backend uses some blocking operations too)
        // If we use Tokio, there would be too many extra wrappers with
        // negligible gain
        thread::Builder::new()
            .name("audio-handler".into())
            .spawn(move || {
                let device = match rodio::DeviceSinkBuilder::open_default_sink() {
                    Ok(d) => d,
                    Err(e) => {
                        eprintln!("audio: no output device: {e}");
                        return;
                    }
                };

                let mut player = rodio::Player::connect_new(device.mixer());

                loop {
                    match rx.recv_timeout(Duration::from_millis(250)) {
                        Ok(cmd) => match cmd {
                            AudioCommand::Load(path) => {
                                player = rodio::Player::connect_new(device.mixer());
                                player.set_volume(handler.real_time.info.volume);
                                player.pause();

                                match File::open(&path).map_err(|e| e.to_string()).and_then(|f| {
                                    rodio::Decoder::try_from(f).map_err(|e| e.to_string())
                                }) {
                                    Ok(source) => {
                                        let tagged_file = Probe::open(path)
                                            .expect("ERROR: Bad path provided!")
                                            .read()
                                            .expect("ERROR: Failed to read file!");
                                        let duration = tagged_file.properties().duration();
                                        player.append(source);
                                        let _ = app.emit(
                                            "track-loaded",
                                            TrackLoadedAudioInfo {
                                                duration_ms: duration.as_millis() as u64,
                                            },
                                        );
                                    }
                                    Err(e) => eprintln!("audio: load failed: {e}"),
                                }
                            }
                            AudioCommand::Toggle => {
                                if player.is_paused() {
                                    player.play();
                                } else {
                                    player.pause();
                                }
                            }
                            AudioCommand::Seek(dur) => {
                                player.try_seek(dur).expect("Couldn't seek the position");
                            }
                            AudioCommand::SetVolume(v) => {
                                handler.real_time.info.volume = v;
                                handler.real_time.info.muted = false;
                                player.set_volume(handler.real_time.info.volume);
                            }
                            AudioCommand::Mute => {
                                handler.real_time.info.muted = !handler.real_time.info.muted;
                                player.set_volume(if handler.real_time.info.muted {
                                    0.0
                                } else {
                                    handler.real_time.info.volume
                                });
                            }
                        },
                        Err(mpsc::RecvTimeoutError::Timeout) => {}
                        Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    }

                    let _ = app.emit(
                        "audio-realtime",
                        RealTimeAudioInfo {
                            progress: Progress {
                                position_ms: if player.empty() {
                                    0
                                } else {
                                    player.get_pos().as_millis() as u64
                                },
                                finished: player.empty(),
                            },
                            info: AudioInfo {
                                volume: handler.real_time.info.volume,
                                muted: handler.real_time.info.muted,
                            },
                        },
                    );
                }
            })
            .expect("Audio handler failed");
        handler
    }

    pub fn send(&self, cmd: AudioCommand) {
        self.tx
            .send(cmd)
            .expect("Failed to send cmd into audio handler");
    }
}
