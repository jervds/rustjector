use crate::core::injector::Injector;
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
    let injector = Injector::new(5, scenario);

    injector
        .inject()
        .await //TODO migrate to stream
        .into_iter()
        .map(|it| match it {
            None => {
                println!("is none !")
            }
            Some(metric) => {
                println!(
                    "user_id: {}, Duration: {}",
                    metric.user_id,
                    metric.duration.as_millis()
                )
            }
        })
        .for_each(drop);

    Ok(())
}
