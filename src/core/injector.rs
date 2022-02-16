use futures::stream::FuturesUnordered;
use futures::StreamExt;

use crate::core::metric::Metric;
use crate::core::scenario::scenario;

pub struct Injector {
    pub(crate) metrics: Vec<Option<Metric>>,
    pub(crate) vus: u32,
}

pub async fn inject(mut injector: Injector) -> anyhow::Result<Injector> {
    let mut futs = FuturesUnordered::new();
    for user_id in 1..=injector.vus {
        futs.push(tokio::spawn(scenario(user_id)));
    }
    while let Some(handled) = futs.next().await {
        let metric = handled??;
        injector.metrics.push(Some(metric))
    }
    Ok(injector)
}
