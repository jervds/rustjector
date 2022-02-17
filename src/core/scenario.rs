use std::time::Instant;

use log::info;

use crate::core::metric::Metric;
use crate::http::http_method::HttpMethod;

#[derive(Clone)]
pub struct Scenario {
    pub(crate) method: HttpMethod,
    pub(crate) url: &'static str,
}

impl Scenario {
    async fn perform_query(&self) -> anyhow::Result<String> {
        Ok(match &self.method {
            HttpMethod::Get => reqwest::get(self.url).await?.text().await?,
        })
    }

    pub async fn execute(scenario: Scenario, user_id: u32) -> anyhow::Result<Metric> {
        let start = Instant::now();
        scenario.perform_query().await?;
        let duration = start.elapsed();
        info!(
            "OS Thread {:?} - for vu {} done in: {:?}",
            std::thread::current().id(),
            user_id,
            start.elapsed(),
        );
        Ok(Metric { duration, user_id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_simple_get_scenario() -> Scenario {
        Scenario {
            method: HttpMethod::Get,
            //TODO mock me!
            url: "https://www.google.com",
        }
    }

    #[tokio::test]
    async fn perform_query_for_get_should_return_response() {
        let response = a_simple_get_scenario().perform_query().await;
        assert_eq!(response.is_ok(), true)
    }

    #[tokio::test]
    async fn execute_should_return_metric() {
        let metric = Scenario::execute(a_simple_get_scenario(), 1).await;
        assert_eq!(metric.is_ok(), true);
        assert_eq!(metric.unwrap().user_id, 1);
        //TODO assert on duration
    }
}
