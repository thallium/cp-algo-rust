use std::{
    fmt,
    iter::{Product, Sum},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
};

use crate::traits::numeric::*;


// Skipped:
//
// - `is_signed_int_t<T>`   (probably won't be used directly in `modint.rs`)
// - `is_unsigned_int_t<T>` (probably won't be used directly in `modint.rs`)
// - `to_unsigned_t<T>`     (not used in `fenwicktree.rs`)

/// Corresponds to `std::is_integral` in C++.
// We will remove unnecessary bounds later.
//
// Maybe we should rename this to `PrimitiveInteger` or something, as it probably won't be used in the
// same way as the original ACL.
pub trait Integral:
    'static
    + Send
    + Sync
    + Copy
    + Ord
    + Not<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + Sum
    + Product
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + BitOrAssign
    + BitAndAssign
    + BitXorAssign
    + Shl<Output = Self>
    + Shr<Output = Self>
    + ShlAssign
    + ShrAssign
    + fmt::Display
    + fmt::Debug
    + fmt::Binary
    + fmt::Octal
    + Zero
    + One
    + BoundedBelow
    + BoundedAbove
{
}
