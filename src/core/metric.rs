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

    pub fn try_to_string(option: Option<Metric>) -> String {
        match option {
            None => "is none !".to_string(),
            Some(a_metric) => {
                format!(
                    "user_id: {}, Duration: {}",
                    a_metric.user_id,
                    a_metric.duration.as_millis()
                )
            }
        }
    }
}
