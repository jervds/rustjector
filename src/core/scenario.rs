use std::time::Instant;

use crate::core::metric::Metric;
use crate::http::http_method::HttpMethod;

pub async fn scenario(user_id: u32, scenario: Scenario) -> anyhow::Result<Metric> {
    let start = Instant::now();
    scenario.perform_query().await?;
    let duration = start.elapsed();
    println!(
        "OS Thread {:?} - for vu {} done in: {:?}",
        std::thread::current().id(),
        user_id,
        start.elapsed(),
    );

    Ok(Metric { duration, user_id })
}

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
}
