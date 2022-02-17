use futures::stream::FuturesUnordered;
use futures::StreamExt;

use crate::core::metric::Metric;
use crate::core::scenario::Scenario;

pub struct Injector {
    pub(crate) vus: u32,
    pub(crate) scenario: Scenario,
}

impl Injector {
    pub async fn inject(self) -> anyhow::Result<Vec<Option<Metric>>> {
        let mut futs = FuturesUnordered::new();
        let mut metrics = Vec::<Option<Metric>>::new();

        for user_id in 1..=self.vus {
            futs.push(tokio::spawn(Scenario::execute(
                self.scenario.clone(),
                user_id,
            )));
        }

        while let Some(handled) = futs.next().await {
            let metric = handled??;
            metrics.push(Some(metric))
        }

        Ok(metrics)
    }
}
