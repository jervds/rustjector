use std::time::Duration;

pub struct Metric {
    pub(crate) duration: Duration,
    pub(crate) user_id: u32,
}
