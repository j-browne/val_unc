//! A package for handling quantities with uncertainties.
//!
//! The [`ValUnc`] type represents a quantity with a mean value `val` and
//! uncertainties `unc`. It is designed to be used with [newtypes] that wrap a
//! basic numeric type, e.g. `f64`. This allows for the type to define how
//! uncertainties should be propagated, with minimal confusion.
//!
//! [newtypes]: https://doc.rust-lang.org/1.0.0/style/features/types/newtype.html
//!
//! The [`traits`] module defines some traits that are necessary for
//! uncertainty types to be implemented in order for the related type to be
//! implemented for `ValUnc`. For example, in order to implement `Add` for
//! `ValUnc`, all of the uncertainty types used must implement [`UncAdd`].
//! These are opt-in and only the traits used need to be implemented.
//!
//! # Features
//!
//! The `serde` feature can be enabled for use with [`serde`]. A `ValUnc<V, U>`
//! is (de)serialized as a `(V, U)` or if `unc` is zero, according to
//! [`num-traits::Zero`], just a `V`.
//!
//! [`serde`]: https://serde.rs
//! [`num-traits::Zero`]: https://docs.rs/num-traits/*/num_traits/identities/trait.Zero.html
//!
//! # Examples
//!
//! The following demonstrates how one would go about creating uncertainty
//! types and implementing the traits necessary for doing math with ValUnc
//! (only `Add`, in this case). Notably, the implementations of `UncAdd` are
//! different. The two uncertainties, though, can be used together in one
//! ValUnc.
//!
//! ```
//! use val_unc::{ValUnc, UncAdd};
//!
//! // This is a type for statistical uncertainties.
//! // The result of adding two `StatUnc`s is the square root of the sum of the squares.
//! #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
//! struct StatUnc(pub f64);
//!
//! impl<T> UncAdd<T> for StatUnc {
//!     fn unc_add(self, _self_val: T, other: Self, _other_val: T) -> Self {
//!         Self(f64::sqrt(f64::powi(self.0, 2) + f64::powi(other.0, 2)))
//!     }
//! }
//!
//! // This is a type for systematic uncertainties.
//! // The result of adding two `SysUnc`s is the sum of the two.
//! #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
//! struct SysUnc(pub f64);
//!
//! impl<T> UncAdd<T> for SysUnc {
//!     fn unc_add(self, _self_val: T, other: Self, _other_val: T) -> Self {
//!         Self(self.0 + other.0)
//!     }
//! }
//!
//! // Create two values and add them together
//! let v1 = ValUnc::new(10.2, (StatUnc(4.0), SysUnc(1.25)));
//! let v2 = ValUnc::new(8.5, (StatUnc(3.0), SysUnc(1.25)));
//! // You can use destructuring to unpack the results
//! let ValUnc { val, unc: (stat, sys) } = v1 + v2;
//!
//! assert!(f64::abs(val - 18.7) <= std::f64::EPSILON);
//! assert!(f64::abs(stat.0 - 5.0) <= std::f64::EPSILON);
//! assert!(f64::abs(sys.0 - 2.5) <= std::f64::EPSILON);
//! ```
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    convert::From,
    ops::{Add, Div, Mul, Neg, Sub},
};

pub mod traits;
pub use traits::*;

#[cfg(feature = "serde")]
mod serde_conversion;

/// A type with a value and uncertainties.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        into = "serde_conversion::ValUncTuple<V, U>",
        from = "serde_conversion::ValUncTuple<V, U>",
        bound(
            serialize = "V: Clone + Serialize, U: Clone + Serialize + UncZero",
            deserialize = "V: Deserialize<'de>, U: Deserialize<'de> + Default"
        )
    )
)]
pub struct ValUnc<V, U> {
    pub val: V,
    pub unc: U,
}

impl<V, U> ValUnc<V, U> {
    pub fn new(val: V, unc: U) -> Self {
        Self { val, unc }
    }
}

impl<V, U> From<V> for ValUnc<V, U>
where
    U: Default,
{
    fn from(val: V) -> Self {
        Self {
            val,
            unc: Default::default(),
        }
    }
}

impl<V, U> Add for ValUnc<V, U>
where
    V: Add<V, Output = V> + Copy,
    U: UncAdd<V>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            val: self.val.add(other.val),
            unc: self.unc.unc_add(self.val, other.unc, other.val),
        }
    }
}

impl<V, U> Div for ValUnc<V, U>
where
    V: Div<V, Output = V> + Copy,
    U: UncDiv<V>,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            val: self.val.div(other.val),
            unc: self.unc.unc_div(self.val, other.unc, other.val),
        }
    }
}

impl<V, U> Mul for ValUnc<V, U>
where
    V: Mul<V, Output = V> + Copy,
    U: UncMul<V>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            val: self.val.mul(other.val),
            unc: self.unc.unc_mul(self.val, other.unc, other.val),
        }
    }
}

impl<V, U> Neg for ValUnc<V, U>
where
    V: Neg<Output = V> + Copy,
    U: UncNeg<V>,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            val: self.val.neg(),
            unc: self.unc.unc_neg(self.val),
        }
    }
}

impl<V, U> Sub for ValUnc<V, U>
where
    V: Sub<V, Output = V> + Copy,
    U: UncSub<V>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            val: self.val.sub(other.val),
            unc: self.unc.unc_sub(self.val, other.unc, other.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
        struct StatUnc(pub f64);

        impl<T> UncAdd<T> for StatUnc {
            fn unc_add(self, _self_val: T, other: Self, _other_val: T) -> Self {
                Self(f64::sqrt(f64::powi(self.0, 2) + f64::powi(other.0, 2)))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
        struct SysUnc(pub f64);

        impl<T> UncAdd<T> for SysUnc {
            fn unc_add(self, _self_val: T, other: Self, _other_val: T) -> Self {
                Self(self.0 + other.0)
            }
        }

        // Create two values and add them together
        let v1 = ValUnc::new(10.2, (StatUnc(4.0), SysUnc(1.25)));
        let v2 = ValUnc::new(8.5, (StatUnc(3.0), SysUnc(1.25)));
        // You can use destructuring to unpack the results
        let ValUnc {
            val,
            unc: (stat, sys),
        } = v1 + v2;

        assert!(f64::abs(val - 18.7) <= std::f64::EPSILON);
        assert!(f64::abs(stat.0 - 5.0) <= std::f64::EPSILON);
        assert!(f64::abs(sys.0 - 2.5) <= std::f64::EPSILON);
    }
}
