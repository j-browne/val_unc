#![feature(custom_attribute)]
use rand::{
    distributions::{Distribution, Normal},
    Rng,
};
use std::convert::From;

#[cfg(feature = "serde-1")]
mod serde;

#[derive(Debug, Clone, Copy)]
pub struct ValUnc {
    pub val: f64,
    pub unc: f64,
}

impl ValUnc {
    pub fn rand<R: Rng>(&self, rng: &mut R) -> ValUnc {
        if self.unc == 0.0 {
            *self
        } else {
            Self {
                val: Normal::new(self.val, self.unc.abs()).sample(rng),
                unc: self.unc,
            }
        }
    }
}

impl From<f64> for ValUnc {
    fn from(val: f64) -> Self {
        Self { val, unc: 0.0 }
    }
}

impl From<(f64, f64)> for ValUnc {
    fn from(tup: (f64, f64)) -> Self {
        Self {
            val: tup.0,
            unc: tup.1,
        }
    }
}
