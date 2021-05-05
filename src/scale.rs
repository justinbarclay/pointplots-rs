//! Transformations between domain and range.

use std::ops::Range;

/// Holds mapping between domain and range of the function.
pub struct Scale {
    domain: Range<f64>,
    range: Range<f64>,
}

impl Scale {
    /// Translates value from domain to range scale.
    /// ```
    /// # use textplots::scale::Scale;
    /// assert_eq!(-0.8, Scale::new(0_f64..10_f64, -1_f64..1_f64).linear(1.0));
    /// ```
    pub fn linear(&self, x: f64) -> f64 {
        let p = (x - self.domain.start) / (self.domain.end - self.domain.start);
        let r = self.range.start + p * (self.range.end - self.range.start);
        r.max(self.range.start).min(self.range.end)
    }

    /// Translates value from range to domain scale.
    /// ```
    /// # use textplots::scale::Scale;
    /// assert_eq!(5.5, Scale::new(0_f64..10_f64, -1_f64..1_f64).inv_linear(0.1));
    /// ```
    pub fn inv_linear(&self, i: f64) -> f64 {
        let p = (i - self.range.start) / (self.range.end - self.range.start);
        let d = self.domain.start + p * (self.domain.end - self.domain.start);
        d.max(self.domain.start).min(self.domain.end)
    }

    pub fn new(domain: Range<f64>, range: Range<f64>) -> Self {
        Scale { domain, range }
    }
}
