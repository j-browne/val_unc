use crate::traits::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A type with a value and uncertainties.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
