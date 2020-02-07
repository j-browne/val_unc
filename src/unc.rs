use crate::traits::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul};

/// An example implementation of an uncertatinty type
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Unc<T>(pub T);

impl<T> From<T> for Unc<T> {
    fn from(val: T) -> Self {
        Self(val)
    }
}

impl<T, U> Mul<T> for Unc<U>
where
    U: Mul<T, Output = U>,
{
    type Output = Self;
    fn mul(self, other: T) -> Self {
        Self(self.0 * other)
    }
}

impl<T, U> Div<T> for Unc<U>
where
    U: Div<T, Output = U>,
{
    type Output = Self;
    fn div(self, other: T) -> Self {
        Self(self.0 / other)
    }
}

impl<V, U> UncAdd<V> for Unc<U>
where
    U: Pow<u8, Output = U> + Sqrt + Add<U, Output = U>,
{
    fn unc_add(self, _self_val: V, other: Unc<U>, _other_val: V) -> Unc<U> {
        Unc((self.0.pow(2) + other.0.pow(2)).sqrt())
    }
}

impl<V, U> UncDiv<V> for Unc<U>
where
    U: Pow<u8, Output = U> + Sqrt + Add<U, Output = U> + Div<V, Output = U> + Mul<V, Output = U>,
    V: Clone,
{
    fn unc_div(self, self_val: V, other: Unc<U>, other_val: V) -> Unc<U> {
        Unc(
            ((self.0 / self_val.clone()).pow(2) + (other.0 / other_val.clone()).pow(2)).sqrt()
                * self_val
                / other_val,
        )
    }
}

impl<V, U> UncMul<V> for Unc<U>
where
    U: Pow<u8, Output = U> + Sqrt + Add<U, Output = U> + Div<V, Output = U> + Mul<V, Output = U>,
    V: Clone,
{
    fn unc_mul(self, self_val: V, other: Unc<U>, other_val: V) -> Unc<U> {
        Unc(
            ((self.0 / self_val.clone()).pow(2) + (other.0 / other_val.clone()).pow(2)).sqrt()
                * self_val
                * other_val,
        )
    }
}

impl<V, U> UncSub<V> for Unc<U>
where
    U: Pow<u8, Output = U> + Sqrt + Add<U, Output = U>,
{
    fn unc_sub(self, _self_val: V, other: Unc<U>, _other_val: V) -> Unc<U> {
        Unc((self.0.pow(2) + other.0.pow(2)).sqrt())
    }
}

impl<T> UncZero for Unc<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Unc(Zero::zero())
    }

    fn is_zero(&self) -> bool {
        Zero::is_zero(&self.0)
    }

    fn set_zero(&mut self) {
        Zero::set_zero(&mut self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unc_f64() {
        let unc_1 = Unc(3.0f64);
        let unc_2 = Unc(4.0f64);

        assert!(
            f64::abs(UncAdd::<f64>::unc_add(unc_1, 1.0, unc_2, 1.0).0 - 5.0) <= std::f64::EPSILON
        );
        assert!(
            f64::abs(UncDiv::<f64>::unc_div(unc_1, 1.0, unc_2, 1.0).0 - 5.0) <= std::f64::EPSILON
        );
        assert!(
            f64::abs(UncMul::<f64>::unc_mul(unc_1, 1.0, unc_2, 1.0).0 - 5.0) <= std::f64::EPSILON
        );
        assert!(
            f64::abs(UncSub::<f64>::unc_sub(unc_1, 1.0, unc_2, 1.0).0 - 5.0) <= std::f64::EPSILON
        );
        assert!(f64::abs(<Unc<f64>>::zero().0 - 0.0) <= std::f64::EPSILON);
    }
}
