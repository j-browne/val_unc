#![feature(custom_attribute)]
use rand::{
    distributions::{Distribution, Normal},
    Rng,
};
use std::convert::From;
use num;

#[cfg(feature = "serde-1")]
mod serde;

/// A structure meant to hold a measurement that has a central value and an
/// uncertainty associated with it.
#[derive(Debug, Clone, Copy)]
pub struct ValUnc<T> {
    /// The central value of the measurement
    pub val: T,
    /// The uncertainty of the measurement
    pub unc: T,
}

impl ValUnc<f64> {
    pub fn rand<R: Rng>(&self, rng: &mut R) -> ValUnc<f64> {
        if self.unc == num::zero() {
            *self
        } else {
            Self {
                val: Normal::new(self.val, self.unc.abs()).sample(rng),
                unc: self.unc,
            }
        }
    }
}

impl<T> From<T> for ValUnc<T> where T: num::Zero {
    fn from(val: T) -> Self {
        Self { val, unc: num::zero() }
    }
}

impl<T> From<(T, T)> for ValUnc<T> {
    fn from(tup: (T, T)) -> Self {
        Self {
            val: tup.0,
            unc: tup.1,
        }
    }
}
