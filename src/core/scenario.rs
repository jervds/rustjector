use std::time::Instant;

use crate::core::metric::Metric;

pub async fn scenario(user_id: u32) -> anyhow::Result<Metric> {
    let start = Instant::now();
    reqwest::get("https://www.google.com").await?.text().await?;
    let duration = start.elapsed();
    println!(
        "OS Thread {:?} - for vu {} done in: {:?}",
        std::thread::current().id(),
        user_id,
        start.elapsed(),
    );

    Ok(Metric { duration, user_id })
}
