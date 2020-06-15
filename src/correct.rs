use crate::{num::Num, num::SignedNum};
use std::ops::*;

#[derive(Debug, Copy, Clone, Eq)]
pub struct Correct<T: Num>(pub T);

impl<T: Num> Correct<T> {
    pub fn zero() -> Self {
        Self(T::zero())
    }
}

unsafe impl<T: Num> Send for Correct<T> {}

unsafe impl<T: Num> Sync for Correct<T> {}

impl<T: Num> Num for Correct<T> {
    fn zero() -> Self {
        Self(T::zero())
    }

    fn overflowing_shl(self, rhs: u32) -> Self {
        self << rhs
    }

    fn overflowing_shr(self, rhs: u32) -> Self {
        self >> rhs
    }
}

impl<T: Num> From<T> for Correct<T> {
    fn from(v: T) -> Self {
        Correct(v)
    }
}

impl<T: Num, R: PartialEq<T>> PartialEq<R> for Correct<T> {
    fn eq(&self, other: &R) -> bool {
        R::eq(other, &self.0)
    }
}

impl<T: SignedNum> Neg for Correct<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<T: SignedNum> Correct<T> {
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }
    pub fn is_negative(self) -> bool {
        self.0.is_negative()
    }
    pub fn is_positive(self) -> bool {
        self.0.is_positive()
    }
}

impl<T: SignedNum> SignedNum for Correct<T> {
    fn abs(self) -> Self {
        Self(self.0.abs())
    }
    fn is_negative(self) -> bool {
        self.0.is_negative()
    }
    fn is_positive(self) -> bool {
        self.0.is_positive()
    }
}

macro_rules! correct_shs_types_impl {
    ($($t:tt)+) => ($(
        impl<T: Num> Shl<$t> for Correct<T> {
            type Output = Self;
            fn shl(self, rhs: $t) -> Self::Output {
                if rhs >= 8 {
                    Self(T::zero())
                } else {
                    Self(self.0.overflowing_shl(rhs as u32))
                }
            }
        }

        impl<T: Num> Shr<$t> for Correct<T> {
            type Output = Self;
            fn shr(self, rhs: $t) -> Self::Output {
                if rhs >= 8 {
                    Self(T::zero())
                } else {
                    Self(self.0.overflowing_shr(rhs as u32))
                }
            }
        }

        impl<T: Num> ShlAssign<$t> for Correct<T> {
            fn shl_assign(&mut self, rhs: $t) {
                *self = *self << rhs
            }
        }

        impl<T: Num> ShrAssign<$t> for Correct<T> {
            fn shr_assign(&mut self, rhs: $t) {
                *self = *self >> rhs
            }
        }
    )+)
}

correct_shs_types_impl! { usize u32 u64 u128 isize i32 i64 i128 }

macro_rules! correct_self_num_impl {
    ($($t:tt: $f:ident)+) => ($(
        impl<T: Num> $t for Correct<T> {
            type Output = Self;
            fn $f(self, rhs: Self) -> Self::Output {
                Self(T::$f(self.0, rhs.0))
            }
        }

        impl<T: Num> $t<T> for Correct<T> {
            type Output = Self;
            fn $f(self, rhs: T) -> Self::Output {
                Self(T::$f(self.0, rhs))
            }
        }
    )+)
}

correct_self_num_impl! {
    Add:add
    Sub:sub
    Mul:mul
    Div:div
    Rem:rem
    BitOr:bitor
    BitAnd:bitand
    BitXor:bitxor
}

macro_rules! correct_self_assign_num_impl {
    ($($t:tt: $f:ident)+) => ($(
        impl<T: Num> $t for Correct<T> {
            fn $f(&mut self, rhs: Self) {
                T::$f(&mut self.0, rhs.0)
            }
        }

        impl<T: Num> $t<T> for Correct<T> {
            fn $f(&mut self, rhs: T) {
                T::$f(&mut self.0, rhs)
            }
        }
    )+)
}

correct_self_assign_num_impl! {
    AddAssign:add_assign
    SubAssign:sub_assign
    MulAssign:mul_assign
    DivAssign:div_assign
    RemAssign:rem_assign
    BitOrAssign:bitor_assign
    BitAndAssign:bitand_assign
    BitXorAssign:bitxor_assign
}
