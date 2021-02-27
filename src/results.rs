use average::{define_histogram, Histogram10, Max, Mean, Min, Quantile, Variance};

#[derive(sqlx::Type)]
pub struct Results {
    units: Units,
    results: Vec<f64>,
}

#[derive(sqlx::Type)]
pub enum Units {
    Milliseconds,
    Microseconds,
    Nanoseconds,
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

impl Results {
    fn new(units: Units, results: Vec<f64>) -> Self {
        Self { units, results }
    }

    fn min(&self) -> f64 {
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
