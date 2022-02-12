use crate::core::injector::Injector;

mod core;

fn main() -> anyhow::Result<()> {
    let injector = Injector {
        url: "https://www.google.com".to_string(),
        metrics: vec![],
        vus: 5,
    };

    let result = injector.inject().unwrap();
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

    Ok(())
}
