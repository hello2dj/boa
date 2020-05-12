//! BigInt primitive

/// FIXME: Temporary spot for BigInt structure
use gc::{unsafe_empty_trace, Finalize, Trace};
// use std::ops::{Deref, DerefMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq)]
pub struct BigInt(num_bigint::BigInt);

impl BigInt {
    #[inline]
    pub fn into_inner(self) -> num_bigint::BigInt {
        self.0
    }

    #[inline]
    pub fn from_str_radix(buf: &str, radix: u32) -> Option<Self> {
        num_bigint::BigInt::parse_bytes(buf.as_bytes(), radix).map(Self)
    }
}

impl From<i64> for BigInt {
    fn from(n: i64) -> BigInt {
        BigInt(num_bigint::BigInt::from(n))
    }
}

macro_rules! impl_bigint_operator {
    ($op:ident, $op_method:ident, $assign_op:ident, $assign_op_method:ident) => {
        impl std::ops::$op for BigInt {
            type Output = Self;

            fn $op_method(mut self, other: Self) -> Self {
                std::ops::$assign_op::$assign_op_method(&mut self.0, other.0);
                self
            }
        }
    };
}

impl_bigint_operator!(Add, add, AddAssign, add_assign);
impl_bigint_operator!(Sub, sub, SubAssign, sub_assign);
impl_bigint_operator!(Mul, mul, MulAssign, mul_assign);
impl_bigint_operator!(Div, div, DivAssign, div_assign);
impl_bigint_operator!(Rem, rem, RemAssign, rem_assign);
impl_bigint_operator!(BitAnd, bitand, BitAndAssign, bitand_assign);
impl_bigint_operator!(BitOr, bitor, BitOrAssign, bitor_assign);
impl_bigint_operator!(BitXor, bitxor, BitXorAssign, bitxor_assign);

impl std::ops::Shr for BigInt {
    type Output = Self;

    fn shr(mut self, other: Self) -> Self::Output {
        use num_traits::cast::ToPrimitive;
        use std::ops::ShlAssign;
        use std::ops::ShrAssign;

        if let Some(n) = other.0.to_i32() {
            if n > 0 {
                self.0.shr_assign(n as usize)
            } else {
                self.0.shl_assign(n.abs() as usize)
            }

            return self;
        }

        panic!("RangeError: Maximum BigInt size exceeded");
    }
}

impl std::ops::Shl for BigInt {
    type Output = Self;

    fn shl(mut self, other: Self) -> Self::Output {
        use num_traits::cast::ToPrimitive;
        use std::ops::ShlAssign;
        use std::ops::ShrAssign;

        if let Some(n) = other.0.to_i32() {
            if n > 0 {
                self.0.shl_assign(n as usize)
            } else {
                self.0.shr_assign(n.abs() as usize)
            }

            return self;
        }

        panic!("RangeError: Maximum BigInt size exceeded");
    }
}

impl std::ops::Neg for BigInt {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl PartialEq<i32> for BigInt {
    fn eq(&self, other: &i32) -> bool {
        self.0 == num_bigint::BigInt::from(*other)
    }
}

impl PartialEq<BigInt> for i32 {
    fn eq(&self, other: &BigInt) -> bool {
        num_bigint::BigInt::from(*self) == other.0
    }
}

impl PartialEq<f64> for BigInt {
    fn eq(&self, other: &f64) -> bool {
        self.0 == num_bigint::BigInt::from(*other as i64)
    }
}

impl PartialEq<BigInt> for f64 {
    fn eq(&self, other: &BigInt) -> bool {
        num_bigint::BigInt::from(*self as i64) == other.0
    }
}

impl std::fmt::Debug for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for BigInt {
    type Target = num_bigint::BigInt;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for BigInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Finalize for BigInt {}
unsafe impl Trace for BigInt {
    unsafe_empty_trace!();
}
