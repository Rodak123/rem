/// Information that has tendencies
/// to change multiple times in short time
#[derive(Clone, serde::Serialize)]
pub struct AudioRealTimeInfo {
    pub position: u64,
    pub finished: bool,
}
impl AudioRealTimeInfo {
    pub fn new() -> Self {
        Self {
            finished: false,
            position: 0,
        }
    }
}
