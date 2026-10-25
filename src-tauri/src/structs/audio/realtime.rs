/// Information that has tendencies
/// to change multiple times in short time
#[derive(Clone, serde::Serialize, Default)]
pub struct AudioRealTimeInfo {
    pub position: u64,
    pub finished: bool,
}
