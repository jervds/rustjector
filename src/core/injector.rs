use std::time::{Duration, Instant};

pub struct Injector {
    pub(crate) url: String,
    pub(crate) duration: Option<Duration>,
}

impl Injector {
    pub(crate) fn inject(self) -> anyhow::Result<Injector> {
        let start = Instant::now();
        reqwest::blocking::get(&self.url)?.text()?;
        let duration = start.elapsed();
        Ok(Self {
            duration: Some(duration),
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
        };

        let result = injector.inject();
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn inject_should_return_duration() {
        let injector = Injector {
            url: "https://www.google.com".to_string(),
            duration: None,
        };

        let result = injector.inject().unwrap();
        assert_eq!(result.duration.is_some(), true);
    }
}
