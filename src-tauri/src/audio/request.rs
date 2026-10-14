use std::{path::PathBuf, time::Duration};

/// Request, which should be send to AudioHandler
/// I think they are self-explaining
pub enum AudioRequest {
    // Because we have Toggle there's no need for play/pause
    // If it is needed, implementation should be fairly simple
    Toggle,
    Load(PathBuf),
    SetVolume(f32),
    Seek(Duration),
    Mute,
}
