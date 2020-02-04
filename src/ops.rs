pub trait UncAdd<V> {
    fn unc_add(self, self_val: V, other: Self, other_val: V) -> Self;
}

pub trait UncDiv<V> {
    fn unc_div(self, self_val: V, other: Self, other_val: V) -> Self;
}

pub trait UncMul<V> {
    fn unc_mul(self, self_val: V, other: Self, other_val: V) -> Self;
}

pub trait UncNeg<V> {
    fn unc_neg(self, self_val: V) -> Self;
}

pub trait UncSub<V> {
    fn unc_sub(self, self_val: V, other: Self, other_val: V) -> Self;
}

