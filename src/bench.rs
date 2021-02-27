use chrono::{DateTime, Utc};
use serde_json::Value;

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
}
