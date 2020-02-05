use crate::{traits::UncZero, ValUnc};
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(super) enum ValUncTuple<V, U> {
    NoUnc(V),
    Unc(V, U),
}

impl<V, U> From<ValUncTuple<V, U>> for ValUnc<V, U>
where
    U: Default,
{
    fn from(v: ValUncTuple<V, U>) -> ValUnc<V, U> {
        match v {
            ValUncTuple::NoUnc(val) => val.into(),
            ValUncTuple::Unc(val, unc) => ValUnc { val, unc },
        }
    }
}

impl<V, U> From<ValUnc<V, U>> for ValUncTuple<V, U>
where
    U: UncZero,
{
    fn from(v: ValUnc<V, U>) -> ValUncTuple<V, U> {
        if UncZero::is_zero(&v.unc) {
            ValUncTuple::NoUnc(v.val)
        } else {
            ValUncTuple::Unc(v.val, v.unc)
        }
    }
}
