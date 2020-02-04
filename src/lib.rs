#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use std::{
    convert::From,
    ops::{Add, Div, Mul, Neg, Sub},
};

pub mod ops;
pub use ops::*;

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
pub struct ValUnc<V, U> {
    pub val: V,
    pub unc: U,
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

// This implements the Unc versions of the ops traits for tuples of types that
// implement those traits (up to 12-tuples).
macro_rules! unc_tuples {
    ($({
        $(($idx:tt) -> $T:ident)*
    })+) => {$(
        // In the following impls, the compiler complains about `other` not being used.
        // This is only for the `()` impl 
        // That's why there's the allow.

        #[allow(unused_variables)]
        impl<V, $($T),*> UncAdd<V> for ($($T,)*)
        where
            V: Copy,
            $($T:UncAdd<V>),*
        {
            fn unc_add(self, self_val: V, other: Self, other_val: V) -> Self {
                ($(
                    self.$idx.unc_add(self_val, other.$idx, other_val),
                )*)
            }
        }

        #[allow(unused_variables)]
        impl<V, $($T),*> UncDiv<V> for ($($T,)*)
        where
            V: Copy,
            $($T:UncDiv<V>),*
        {
            fn unc_div(self, self_val: V, other: Self, other_val: V) -> Self {
                ($(
                    self.$idx.unc_div(self_val, other.$idx, other_val),
                )*)
            }
        }

        #[allow(unused_variables)]
        impl<V, $($T),*> UncMul<V> for ($($T,)*)
        where
            V: Copy,
            $($T:UncMul<V>),*
        {
            fn unc_mul(self, self_val: V, other: Self, other_val: V) -> Self {
                ($(
                    self.$idx.unc_mul(self_val, other.$idx, other_val),
                )*)
            }
        }

        #[allow(unused_variables)]
        impl<V, $($T),*> UncNeg<V> for ($($T,)*)
        where
            V: Copy,
            $($T:UncNeg<V>),*
        {
            fn unc_neg(self, self_val: V) -> Self {
                ($(
                    self.$idx.unc_neg(self_val),
                )*)
            }
        }

        #[allow(unused_variables)]
        impl<V, $($T),*> UncSub<V> for ($($T,)*)
        where
            V: Copy,
            $($T:UncSub<V>),*
        {
            fn unc_sub(self, self_val: V, other: Self, other_val: V) -> Self {
                ($(
                    self.$idx.unc_sub(self_val, other.$idx, other_val),
                )*)
            }
        }
    )+}
}

unc_tuples!(
    {
    }
    {
        (0) -> U0
    }
    {
        (0) -> U0
        (1) -> U1
    }
    {
        (0) -> U0
        (1) -> U1
        (2) -> U2
    }
    {
        (0) -> U0
        (1) -> U1
        (2) -> U2
        (3) -> U3
    }
    {
        (0) -> U0
        (1) -> U1
        (2) -> U2
        (3) -> U3
        (4) -> U4
    }
    {
        (0) -> U0
        (1) -> U1
        (2) -> U2
        (3) -> U3
        (4) -> U4
        (5) -> U5
    }
    {
        (0) -> U0
        (1) -> U1
        (2) -> U2
        (3) -> U3
        (4) -> U4
        (5) -> U5
        (6) -> U6
    }
    {
        (0) -> U0
        (1) -> U1
        (2) -> U2
        (3) -> U3
        (4) -> U4
        (5) -> U5
        (6) -> U6
        (7) -> U7
    }
    {
        (0) -> U0
        (1) -> U1
        (2) -> U2
        (3) -> U3
        (4) -> U4
        (5) -> U5
        (6) -> U6
        (7) -> U7
        (8) -> U8
    }
    {
        (0) -> U0
        (1) -> U1
        (2) -> U2
        (3) -> U3
        (4) -> U4
        (5) -> U5
        (6) -> U6
        (7) -> U7
        (8) -> U8
        (9) -> U9
    }
    {
        (0) -> U0
        (1) -> U1
        (2) -> U2
        (3) -> U3
        (4) -> U4
        (5) -> U5
        (6) -> U6
        (7) -> U7
        (8) -> U8
        (9) -> U9
        (10) -> U10
    }
    {
        (0) -> U0
        (1) -> U1
        (2) -> U2
        (3) -> U3
        (4) -> U4
        (5) -> U5
        (6) -> U6
        (7) -> U7
        (8) -> U8
        (9) -> U9
        (10) -> U10
        (11) -> U11
    }
);

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
