use crate::Error;
use average::{Histogram10, Max, Mean, Min, Variance};

#[derive(sqlx::Type)]
#[sqlx(type_name = "results")]
pub struct Results {
    units: Units,
    results: Vec<f64>,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "units")]
#[sqlx(rename_all = "lowercase")]
pub enum Units {
    Milliseconds,
    Microseconds,
    Nanoseconds,
}

impl Results {
    pub fn new(units: Units, results: Vec<f64>) -> Result<Self, Error> {
        if results.is_empty() {
            return Err(Error::EmptyResults);
        }

        Ok(Self { units, results })
    }

    pub fn min(&self) -> f64 {
        let m: Min = self.results.iter().collect();
        m.min()
    }

    fn max(&self) -> f64 {
        let m: Max = self.results.iter().collect();
        m.max()
    }

    pub fn mean(&self) -> f64 {
        let m: Mean = self.results.iter().collect();
        m.mean()
    }

    pub fn variance(&self) -> f64 {
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

impl Into<String> for Units {
    fn into(self) -> String {
        use Units::*;

        (match self {
            Milliseconds => "ms",
            Microseconds => "us",
            Nanoseconds => "ns",
        })
        .to_owned()
    }
}

impl From<&str> for Units {
    fn from(s: &str) -> Units {
        use Units::*;

        match s {
            "milliseconds" | "millis" | "ms" => Milliseconds,
            "microseconds" | "micros" | "us" | "μs" => Microseconds,
            "nanoseconds" | "nanos" | "ns" => Nanoseconds,
            _ => panic!("unit {} not recognized", s),
        }
    }
}

impl Default for Units {
    fn default() -> Self {
        Self::Milliseconds
    }
}

#[cfg(test)]
mod tests {
    use super::{Results, Units};

    #[test]
    fn error_on_empty_results() {
        let results = Results::new(Units::default(), vec![]);
        assert!(results.is_err());
    }

    #[test]
    fn results_min() {
        let res = vec![0.0, 101.1, 497.75];
        let results = Results::new(Units::default(), res).unwrap();
        assert_eq!(results.min(), 0.0);
    }

    #[test]
    fn results_max() {
        let res = vec![1010101.0, 1010101.1];
        let results = Results::new(Units::default(), res).unwrap();
        assert_eq!(results.max(), 1010101.1);
    }

    #[test]
    fn results_mean() {
        let res = vec![1010101.0, 1010101.1];
        let results = Results::new(Units::default(), res).unwrap();
        assert_eq!(results.mean(), 1010101.05);
    }

    #[test]
    fn results_variance() {
        let res = vec![0.0, 1.0, 2.0];
        let results = Results::new(Units::default(), res).unwrap();
        assert_eq!(results.variance(), 1.0);
    }
}
