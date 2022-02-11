use crate::core::injector::Injector;

mod core;

fn main() -> anyhow::Result<()> {
    let injector = Injector {
        url: "https://www.google.com".to_string(),
        duration: None,
    };

    let result = injector.inject().unwrap();
    println!("{}", result.duration.unwrap().as_millis());

    Ok(())
}
