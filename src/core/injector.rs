use futures::stream::FuturesUnordered;
use futures::{StreamExt, TryStreamExt};
use std::sync::Arc;
use tokio::task::JoinHandle;

use crate::core::metric::Metric;
use crate::core::scenario::Scenario;

pub struct Injector {
    pub(crate) vus: usize,
    pub(crate) scenario: Arc<Scenario>,
}

impl Injector {
    pub fn new(vus: usize, scenario: Scenario) -> Self {
        Injector {
            vus,
            scenario: Arc::new(scenario),
        }
    }

    pub async fn inject(self) -> Vec<Option<Metric>> {
        let metrics = self
            .spawn_scenarios()
            .await
            .into_stream()
            .map(Metric::from_scenario_execution)
            .collect();

        metrics.await
    }

    async fn spawn_scenarios(&self) -> FuturesUnordered<JoinHandle<anyhow::Result<Metric>>> {
        let futures = FuturesUnordered::new();

        (1..=self.vus)
            .map(|it| {
                futures.push(tokio::spawn(Scenario::execute(self.scenario.clone(), it)));
            })
            .for_each(drop);
        futures
    }
}
