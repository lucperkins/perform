use crate::results::Results;
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow)]
pub struct Bench {
    pub timestamp: DateTime<Utc>,
    pub mean: f64,
    pub variance: f64,
}

impl Bench {
    pub fn from_results(results: Results) -> Self {
        let timestamp = Utc::now();
        let mean = results.mean();
        let variance = results.variance();

        Self {
            timestamp,
            mean,
            variance,
        }
    }
}
