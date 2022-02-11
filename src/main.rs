use crate::injector::injector::Injector;

mod injector;

fn main() -> anyhow::Result<()> {
    let injector = Injector {
        url: "https://www.google.com".to_string(),
        duration: None,
        response: None
    };

    let result = injector.inject().unwrap();
    println!("{}",result.duration.unwrap().as_millis());

    Ok(())
}
