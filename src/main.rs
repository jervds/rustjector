use crate::core::injector::Injector;
use crate::core::metric::Metric;
use crate::core::scenario::Scenario;
use crate::http::http_method::HttpMethod;

mod core;
mod http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let scenario = Scenario {
        method: HttpMethod::Get,
        url: "https://www.google.com",
    };

    Injector::new(5, scenario)
        .inject()
        .await //TODO migrate to stream
        .into_iter()
        .map(Metric::try_to_string)
        .for_each(|it| println!("{}", it));

    Ok(())
}
