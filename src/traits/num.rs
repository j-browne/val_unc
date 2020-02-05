use num_traits::Zero;

pub trait UncZero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
    fn set_zero(&mut self);
}

macro_rules! unc_zero_impl {
    ($($T:ty),+) => {$(
        impl UncZero for $T {
            fn zero() -> Self {
                Zero::zero()
            }

            fn is_zero(&self) -> bool {
                Zero::is_zero(self)
            }

            fn set_zero(&mut self) {
                Zero::set_zero(self)
            }
        }
    )+}
}

unc_zero_impl!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);

impl UncZero for () {
    fn zero() -> Self {
        ()
    }

    fn is_zero(&self) -> bool {
        true
    }

    fn set_zero(&mut self) {}
}

macro_rules! unc_num_tuples {
    ($({
        $(($idx:tt, $T:ident)),*
    })+) => {$(
        impl<$($T),*> UncZero for ($($T,)*)
        where
            $($T: UncZero),*
        {
            fn zero() -> Self {
                ($($T::zero(),)*)
            }

            fn is_zero(&self) -> bool {
                ($(self.$idx.is_zero())&&*)
            }

            fn set_zero(&mut self) {
                $(self.$idx.set_zero();)*
            }
        }
    )+}
}

unc_num_tuples!(
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
