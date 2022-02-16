use crate::core::injector::{inject, Injector};
use crate::core::scenario::Scenario;
use crate::http::http_method::HttpMethod;

mod core;
mod http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let scenario = Scenario {
        method: HttpMethod::Get,
        url: "https://www.google.com",
    };
    let injector = Injector {
        metrics: vec![],
        vus: 5,
        scenario,
    };

    let result = inject(injector).await?;
    result
        .metrics
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
    //
    // let result = injector.inject().unwrap();
    // result
    //     .metrics
    //     .into_iter()
    //     .map(|it| match it {
    //         None => {
    //             println!("is none !")
    //         }
    //         Some(metric) => {
    //             println!(
    //                 "user_id: {}, Duration: {}",
    //                 metric.user_id,
    //                 metric.duration.as_millis()
    //             )
    //         }
    //     })
    //     .for_each(drop);

    Ok(())
}
