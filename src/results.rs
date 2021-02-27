use average::{define_histogram, Histogram10, Max, Mean, Min, Quantile, Variance};

#[derive(sqlx::Type)]
pub struct Results(Vec<f64>);

impl Results {
    fn new(results: Vec<f64>) -> Self {
        Self(results)
    }

    fn min(&self) -> f64 {
        let m: Min = self.0.iter().collect();
        m.min()
    }

    fn max(&self) -> f64 {
        let m: Max = self.0.iter().collect();
        m.max()
    }

    pub fn mean(&self) -> f64 {
        let m: Mean = self.0.iter().collect();
        m.mean()
    }

    pub fn variance(&self) -> f64 {
        let v: Variance = self.0.iter().collect();
        v.sample_variance()
    }

    fn histogram(&self) -> Histogram10 {
        let mut hist = Histogram10::with_const_width(self.min(), self.max());
        self.0.iter().for_each(|i| {
            let _ = hist.add(*i);
        });
        hist
    }
}
