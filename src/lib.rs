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

/// `ValUnc` is meant to be used with newtypes.
///
/// `{`[`Add`]`, `[`Div`]`, `[`Mul`]`, `[`Neg`]`, `[`Sub`]`}`
/// are implemented if `V` implements that trait
/// and `U` implements `{`[`UncAdd<V>`]`, `[`UncDiv<V>`]`, `[`UncMul<V>`]`,`[`UncNeg<V>`]`, `[`UncSub<V>`]`}`.
///
/// [`UncAdd<V>`]: UncAdd
/// [`UncDiv<V>`]: UncDiv
/// [`UncMul<V>`]: UncMul
/// [`UncNeg<V>`]: UncNeg
/// [`UncSub<V>`]: UncSub
///
///
///
/// # Examples
/// ```
/// use val_unc::{ValUnc, UncAdd};
///
/// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// struct StatUnc(pub f64);
///
/// impl UncAdd<f64> for StatUnc {
///     fn unc_add(self, _self_val: f64, other: StatUnc, _other_val: f64) -> StatUnc {
///         StatUnc(f64::sqrt(f64::powi(self.0, 2) + f64::powi(other.0, 2)))
///     }
/// }
///
/// let v1 = ValUnc {
///     val: 1.0,
///     unc: StatUnc(3.0),
/// };
/// let v2 = ValUnc {
///     val: 5.0,
///     unc: StatUnc(4.0),
/// };
///
/// let ValUnc { val, unc } = v1 + v2;
/// assert!((val - 6.0).abs() <= std::f64::EPSILON);
/// assert!((unc.0 - 5.0).abs() <= std::f64::EPSILON);
/// ```
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
        Self {
            val,
            unc,
        }
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

    /// Add `val`s together using [`std::ops::Add`], and add `unc`s together using [`UncAdd`]
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

    ///
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

    ///
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

    ///
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

    ///
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

        impl UncAdd<f64> for StatUnc {
            fn unc_add(self, _self_val: f64, other: StatUnc, _other_val: f64) -> StatUnc {
                StatUnc(f64::sqrt(f64::powi(self.0, 2) + f64::powi(other.0, 2)))
            }
        }

        let v1 = ValUnc {
            val: 1.0,
            unc: StatUnc(3.0),
        };
        let v2 = ValUnc {
            val: 5.0,
            unc: StatUnc(4.0),
        };

        let ValUnc { val, unc } = v1 + v2;
        assert!((val - 6.0).abs() <= std::f64::EPSILON);
        assert!((unc.0 - 5.0).abs() <= std::f64::EPSILON);
    }
}
