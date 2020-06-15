use std::ops::*;

pub trait Num:
    Copy
    + Send
    + Sync
    + Clone
    + Eq
    + PartialEq
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
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + Shl<u32, Output = Self>
    + Shr<u32, Output = Self>
    + Shl<u64, Output = Self>
    + Shr<u64, Output = Self>
    + Shl<u128, Output = Self>
    + Shr<u128, Output = Self>
    + Shl<isize, Output = Self>
    + Shr<isize, Output = Self>
    + Shl<i32, Output = Self>
    + Shr<i32, Output = Self>
    + Shl<i64, Output = Self>
    + Shr<i64, Output = Self>
    + Shl<i128, Output = Self>
    + Shr<i128, Output = Self>
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + ShlAssign<usize>
    + ShrAssign<usize>
    + ShlAssign<u32>
    + ShrAssign<u32>
    + ShlAssign<u64>
    + ShrAssign<u64>
    + ShlAssign<u128>
    + ShrAssign<u128>
    + ShlAssign<isize>
    + ShrAssign<isize>
    + ShlAssign<i32>
    + ShrAssign<i32>
    + ShlAssign<i64>
    + ShrAssign<i64>
    + ShlAssign<i128>
    + ShrAssign<i128>
    + BitOrAssign
    + BitAndAssign
    + BitXorAssign
    + Sized
{
    fn zero() -> Self;
    fn overflowing_shl(self, rhs: u32) -> Self;
    fn overflowing_shr(self, rhs: u32) -> Self;
}

pub trait SignedNum: Num + Neg<Output = Self> {
    fn abs(self) -> Self;
    fn is_negative(self) -> bool;
    fn is_positive(self) -> bool;
}

macro_rules! num_impl {
    ($($t:ty)+) => ($(
        impl Num for $t {
            fn zero() -> Self {
                0
            }
            fn overflowing_shl(self, rhs: u32) -> Self {
                self.overflowing_shl(rhs).0
            }
            fn overflowing_shr(self, rhs: u32) -> Self {
                self.overflowing_shr(rhs).0
            }
        }
    )+)
}

num_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

macro_rules! signed_num_impl {
    ($($t:ty)+) => ($(
        impl SignedNum for $t {
            fn abs(self) -> Self {
                self.abs()
            }
            fn is_negative(self) -> bool {
                self.is_negative()
            }
            fn is_positive(self) -> bool {
                self.is_positive()
            }
        }
    )+)
}

signed_num_impl! { isize i8 i16 i32 i64 i128 }
