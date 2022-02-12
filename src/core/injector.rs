use std::time::Instant;

use crate::core::metric::Metric;

pub struct Injector {
    pub(crate) url: String,
    pub(crate) metrics: Vec<Option<Metric>>,
    pub(crate) vus: u32,
}

impl Injector {
    pub(crate) fn inject(self) -> anyhow::Result<Injector> {
        self.init()?;
        let mut metrics = Vec::<Option<Metric>>::new();
        for user_id in 1..=self.vus {
            match self.run_scenario(user_id) {
                Ok(metric) => metrics.push(Some(metric)),
                Err(_) => metrics.push(None),
            }
        }

        Ok(Injector { metrics, ..self })
    }

    fn init(&self) -> anyhow::Result<()> {
        //TODO improve me!
        reqwest::blocking::get(&self.url)?.text()?;
        Ok(())
    }

    fn run_scenario(&self, user_id: u32) -> anyhow::Result<Metric> {
        let start = Instant::now();
        reqwest::blocking::get(&self.url)?.text()?;
        let duration = start.elapsed();
        Ok(Metric { duration, user_id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inject_should_return_ok() {
        let injector = Injector {
            url: "https://www.google.com".to_string(),
            metrics: vec![],
            vus: 3,
        };

        let result = injector.inject();
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn inject_should_return_metrics() {
        let injector = Injector {
            url: "https://www.google.com".to_string(),
            metrics: vec![],
            vus: 3,
        };

        let result = injector.inject().unwrap();
        assert_eq!(result.metrics.len(), 3);
    }

    #[test]
    fn run_scenario_should_fill_metric() {
        let injector = Injector {
            url: "https://www.google.com".to_string(),
            metrics: vec![],
            vus: 3,
        };

        let result_metric = injector.run_scenario(1);
        assert_eq!(result_metric.is_ok(), true);
        let metric = result_metric.unwrap();
        assert_eq!(metric.duration.as_millis() > 0, true);
        assert_eq!(metric.user_id, 1);
    }
}
