use crate::traits::integral::Integral;
/// Class that has additive identity element
pub trait Zero {
    /// The additive identity element
    fn zero() -> Self;
}

/// Class that has multiplicative identity element
pub trait One {
    /// The multiplicative identity element
    fn one() -> Self;
}

pub trait BoundedBelow {
    fn min_value() -> Self;
}

pub trait BoundedAbove {
    fn max_value() -> Self;
}

macro_rules! impl_integral {
    ($($ty:ty),*) => {
        $(
            impl Zero for $ty {
                #[inline]
                fn zero() -> Self {
                    0
                }
            }

            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    1
                }
            }

            impl BoundedBelow for $ty {
                #[inline]
                fn min_value() -> Self {
                    Self::min_value()
                }
            }

            impl BoundedAbove for $ty {
                #[inline]
                fn max_value() -> Self {
                    Self::max_value()
                }
            }

            impl Integral for $ty {}
        )*
    };
}

impl_integral!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
