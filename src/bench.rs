use average::{define_histogram, Histogram10, Max, Mean, Min, Quantile, Variance};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;

#[derive(sqlx::FromRow)]
pub struct Bench {
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: Option<Value>,
    pub results: Vec<f64>,
}

impl Bench {
    fn new(name: String, metadata: Option<Value>, results: Vec<f64>) -> Self {
        let now = Utc::now();

        Self {
            name,
            timestamp: now,
            metadata,
            results,
        }
    }

    fn min(&self) -> f64 {
        let m: Min = self.results.iter().collect();
        m.min()
    }

    fn max(&self) -> f64 {
        let m: Max = self.results.iter().collect();
        m.max()
    }

    fn mean(&self) -> f64 {
        let m: Mean = self.results.iter().collect();
        m.mean()
    }

    fn variance(&self) -> f64 {
        let v: Variance = self.results.iter().collect();
        v.sample_variance()
    }

    fn histogram(&self) -> Histogram10 {
        let mut hist = Histogram10::with_const_width(self.min(), self.max());
        self.results.iter().for_each(|i| {
            let _ = hist.add(*i);
        });
        hist
    }
}
