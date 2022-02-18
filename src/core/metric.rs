use std::time::Duration;

use anyhow::Error;
use tokio::task::JoinError;

pub struct Metric {
    pub(crate) duration: Duration,
    pub(crate) user_id: usize,
}

impl Metric {
    pub fn from_scenario_execution(
        result: Result<Result<Metric, Error>, JoinError>,
    ) -> Option<Metric> {
        match result {
            Ok(result1) => match result1 {
                Ok(metric) => Some(metric),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }
}
