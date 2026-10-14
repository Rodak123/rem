use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct AudioOptions {
    pub volume: f32,
    pub muted: bool,
    pub shuffle: bool,
    pub repeat: bool,
    // Maybe someday (it's kind of useless imo)
    // pub speed: bool,
}
impl AudioOptions {
    pub fn new() -> Self {
        Self {
            volume: 1.0,
            muted: false,
            repeat: false,
            shuffle: false,
            // speed: 1
        }
    }
}
