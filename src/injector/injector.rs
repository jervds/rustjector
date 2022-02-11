use std::time::{Duration, Instant};

pub struct Injector {
    pub(crate) url: String,
    pub(crate) duration: Option<Duration>,
    pub(crate) response: Option<String>
}

impl Injector {
    pub(crate) fn inject(self) -> anyhow::Result<Injector> {
        let start = Instant::now();
        let response = reqwest::blocking::get(&self.url)?.text()?;
        let duration = start.elapsed();
        Ok(Self {
            duration: Some(duration),
            response: Some(response),
            ..self
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inject_should_return_ok() {
        let injector = Injector {
            url: "https://www.google.com".to_string(),
            duration: None,
            response: None
        };

        let result = injector.inject();
        assert_eq!(result.is_ok(),true);
    }

    #[test]
    fn inject_should_return_duration() {
        let injector = Injector {
            url: "https://www.google.com".to_string(),
            duration: None,
            response: None
        };

        let result = injector.inject().unwrap();
        assert_eq!(result.duration.is_some(),true);
    }

    #[test]
    fn inject_should_return_response() {
        let injector = Injector {
            url: "https://www.google.com".to_string(),
            duration: None,
            response: None
        };

        let result = injector.inject().unwrap();
        assert_eq!(result.response.is_some(),true);
    }
}