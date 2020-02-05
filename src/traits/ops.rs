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

// This implements the crate::ops traits and num-traits::Zero for tuples of types that implement those traits (up to
// 12-tuples).
macro_rules! unc_ops_tuples {
    ($({
        $(($idx:tt, $T:ident)),*
    })+) => {$(
        // In the following impls, the compiler complains about `other` not being used.
        // This is only for the `()` impl
        // That's why there's the allow.

        #[allow(unused_variables)]
        impl<V, $($T),*> UncAdd<V> for ($($T,)*)
        where
            V: Copy,
            $($T: UncAdd<V>),*
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
            $($T: UncDiv<V>),*
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
            $($T: UncMul<V>),*
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
            $($T: UncNeg<V>),*
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
            $($T: UncSub<V>),*
        {
            fn unc_sub(self, self_val: V, other: Self, other_val: V) -> Self {
                ($(
                    self.$idx.unc_sub(self_val, other.$idx, other_val),
                )*)
            }
        }
    )+}
}

unc_ops_tuples!(
    {}
    {(0, U0)}
    {(0, U0), (1, U1)}
    {(0, U0), (1, U1), (2, U2)}
    {(0, U0), (1, U1), (2, U2), (3, U3)}
    {(0, U0), (1, U1), (2, U2), (3, U3), (4, U4)}
    {(0, U0), (1, U1), (2, U2), (3, U3), (4, U4), (5, U5)}
    {(0, U0), (1, U1), (2, U2), (3, U3), (4, U4), (5, U5), (6, U6)}
    {(0, U0), (1, U1), (2, U2), (3, U3), (4, U4), (5, U5), (6, U6), (7, U7)}
    {(0, U0), (1, U1), (2, U2), (3, U3), (4, U4), (5, U5), (6, U6), (7, U7),
        (8, U8)}
    {(0, U0), (1, U1), (2, U2), (3, U3), (4, U4), (5, U5), (6, U6), (7, U7),
        (8, U8), (9, U9)}
    {(0, U0), (1, U1), (2, U2), (3, U3), (4, U4), (5, U5), (6, U6), (7, U7),
        (8, U8), (9, U9), (10, U10)}
    {(0, U0), (1, U1), (2, U2), (3, U3), (4, U4), (5, U5), (6, U6), (7, U7),
        (8, U8), (9, U9), (10, U10), (11, U11)}
);
