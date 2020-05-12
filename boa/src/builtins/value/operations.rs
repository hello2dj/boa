use super::*;
use crate::builtins::number;
use crate::Interpreter;

use std::borrow::Borrow;
use std::convert::TryFrom;

impl Value {
    /// Strict equality comparison.
    ///
    /// This method is executed when doing strict equality comparisons with the `===` operator.
    /// For more information, check <https://tc39.es/ecma262/#sec-strict-equality-comparison>.
    pub fn strict_equals(&self, other: &Self) -> bool {
        if self.get_type() != other.get_type() {
            return false;
        }

        if self.is_number() {
            return number::equals(f64::from(self), f64::from(other));
        }

        same_value_non_number(self, other)
    }

    /// Abstract equality comparison.
    ///
    /// This method is executed when doing abstract equality comparisons with the `==` operator.
    ///  For more information, check <https://tc39.es/ecma262/#sec-abstract-equality-comparison>
    pub fn equals(&mut self, other: &mut Self, interpreter: &mut Interpreter) -> bool {
        // 1. If Type(x) is the same as Type(y), then
        //     a. Return the result of performing Strict Equality Comparison x === y.
        if self.get_type() == other.get_type() {
            return self.strict_equals(other);
        }

        match (self.data(), other.data()) {
            // 2. If x is null and y is undefined, return true.
            // 3. If x is undefined and y is null, return true.
            _ if self.is_null_or_undefined() && other.is_null_or_undefined() => true,

            // 3. If Type(x) is Number and Type(y) is String, return the result of the comparison x == ! ToNumber(y).
            // 4. If Type(x) is String and Type(y) is Number, return the result of the comparison ! ToNumber(x) == y.
            //
            // https://github.com/rust-lang/rust/issues/54883
            (ValueData::Integer(_), ValueData::String(_))
            | (ValueData::Rational(_), ValueData::String(_))
            | (ValueData::String(_), ValueData::Integer(_))
            | (ValueData::String(_), ValueData::Rational(_))
            | (ValueData::Rational(_), ValueData::Boolean(_))
            | (ValueData::Integer(_), ValueData::Boolean(_)) => {
                let a: &Value = self.borrow();
                let b: &Value = other.borrow();
                number::equals(f64::from(a), f64::from(b))
            }
            (ValueData::Boolean(_), _) => {
                other.equals(&mut Value::from(self.to_integer()), interpreter)
            }
            (_, ValueData::Boolean(_)) => {
                self.equals(&mut Value::from(other.to_integer()), interpreter)
            }
            (ValueData::Object(_), _) => {
                let mut primitive = interpreter.to_primitive(self, None);
                primitive.equals(other, interpreter)
            }
            (_, ValueData::Object(_)) => {
                let mut primitive = interpreter.to_primitive(other, None);
                primitive.equals(self, interpreter)
            }
            _ => false,
        }
    }
}

/// The internal comparison abstract operation SameValue(x, y),
/// where x and y are ECMAScript language values, produces true or false.
/// Such a comparison is performed as follows:
///
/// https://tc39.es/ecma262/#sec-samevalue
/// strict mode currently compares the pointers
pub fn same_value(x: &Value, y: &Value, strict: bool) -> bool {
    if strict {
        // Do both Values point to the same underlying valueData?
        let x_ptr = Gc::into_raw(x.0.clone());
        let y_ptr = Gc::into_raw(y.0.clone());
        return x_ptr == y_ptr;
    }

    if x.get_type() != y.get_type() {
        return false;
    }

    // TODO: check BigInt
    // https://github.com/jasonwilliams/boa/pull/358
    if x.is_number() {
        return number::same_value(f64::from(x), f64::from(y));
    }

    same_value_non_number(x, y)
}

pub fn same_value_non_number(x: &Value, y: &Value) -> bool {
    debug_assert!(x.get_type() == y.get_type());
    match x.get_type() {
        "undefined" => true,
        "null" => true,
        "string" => {
            if x.to_string() == y.to_string() {
                return true;
            }
            false
        }
        "bigint" => BigInt::try_from(x).unwrap() == BigInt::try_from(y).unwrap(),
        "boolean" => bool::from(x) == bool::from(y),
        "object" => std::ptr::eq(x, y),
        _ => false,
    }
}

impl Add for Value {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self.data(), other.data()) {
            (ValueData::String(ref s), ref o) => {
                Self::string(format!("{}{}", s.clone(), &o.to_string()))
            }
            (ValueData::BigInt(ref n1), ValueData::BigInt(ref n2)) => {
                Self::bigint(n1.clone() + n2.clone())
            }
            (ref s, ValueData::String(ref o)) => Self::string(format!("{}{}", s.to_string(), o)),
            (ref s, ref o) => Self::rational(s.to_number() + o.to_number()),
        }
    }
}
impl Sub for Value {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        match (self.data(), other.data()) {
            (ValueData::BigInt(ref a), ValueData::BigInt(ref b)) => {
                Self::bigint(a.clone() - b.clone())
            }
            (a, b) => Self::rational(a.to_number() - b.to_number()),
        }
    }
}
impl Mul for Value {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match (self.data(), other.data()) {
            (ValueData::BigInt(ref a), ValueData::BigInt(ref b)) => {
                Self::bigint(a.clone() * b.clone())
            }
            (a, b) => Self::rational(a.to_number() * b.to_number()),
        }
    }
}
impl Div for Value {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        match (self.data(), other.data()) {
            (ValueData::BigInt(ref a), ValueData::BigInt(ref b)) => {
                Self::bigint(a.clone() / b.clone())
            }
            (a, b) => Self::rational(a.to_number() / b.to_number()),
        }
    }
}
impl Rem for Value {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        match (self.data(), other.data()) {
            (ValueData::BigInt(ref a), ValueData::BigInt(ref b)) => {
                Self::bigint(a.clone() % b.clone())
            }
            (a, b) => Self::rational(a.to_number() % b.to_number()),
        }
    }
}
impl BitAnd for Value {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        match (self.data(), other.data()) {
            (ValueData::BigInt(ref a), ValueData::BigInt(ref b)) => {
                Self::bigint(a.clone() & b.clone())
            }
            (a, b) => Self::integer(a.to_integer() & b.to_integer()),
        }
    }
}
impl BitOr for Value {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        match (self.data(), other.data()) {
            (ValueData::BigInt(ref a), ValueData::BigInt(ref b)) => {
                Self::bigint(a.clone() | b.clone())
            }
            (a, b) => Self::integer(a.to_integer() | b.to_integer()),
        }
    }
}
impl BitXor for Value {
    type Output = Self;
    fn bitxor(self, other: Self) -> Self {
        match (self.data(), other.data()) {
            (ValueData::BigInt(ref a), ValueData::BigInt(ref b)) => {
                Self::bigint(a.clone() ^ b.clone())
            }
            (a, b) => Self::integer(a.to_integer() ^ b.to_integer()),
        }
    }
}

impl Shl for Value {
    type Output = Self;
    fn shl(self, other: Self) -> Self {
        match (self.data(), other.data()) {
            (ValueData::BigInt(ref a), ValueData::BigInt(ref b)) => {
                Self::bigint(a.clone() << b.clone())
            }
            (a, b) => Self::integer(a.to_integer() << b.to_integer()),
        }
    }
}
impl Shr for Value {
    type Output = Self;
    fn shr(self, other: Self) -> Self {
        match (self.data(), other.data()) {
            (ValueData::BigInt(ref a), ValueData::BigInt(ref b)) => {
                Self::bigint(a.clone() >> b.clone())
            }
            (a, b) => Self::integer(a.to_integer() >> b.to_integer()),
        }
    }
}
impl Not for Value {
    type Output = Self;
    fn not(self) -> Self {
        Self::boolean(!self.is_true())
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.data() {
            ValueData::Object(_) | ValueData::Symbol(_) | ValueData::Undefined => {
                Self::rational(NAN)
            }
            ValueData::String(ref str) => Self::rational(match f64::from_str(str) {
                Ok(num) => -num,
                Err(_) => NAN,
            }),
            ValueData::Rational(num) => Self::rational(-num),
            ValueData::Integer(num) => Self::rational(-f64::from(*num)),
            ValueData::Boolean(true) => Self::integer(1),
            ValueData::Boolean(false) | ValueData::Null => Self::integer(0),
            ValueData::BigInt(ref num) => Self::bigint(-num.clone()),
        }
    }
}

/// The internal comparison abstract operation SameValueZero(x, y),
/// where x and y are ECMAScript language values, produces true or false.
/// SameValueZero differs from SameValue only in its treatment of +0 and -0.
///
/// Such a comparison is performed as follows:
///
/// <https://tc39.es/ecma262/#sec-samevaluezero>
pub fn same_value_zero(x: &Value, y: &Value) -> bool {
    if x.get_type() != y.get_type() {
        return false;
    }

    if x.is_number() {
        return number::same_value_zero(f64::from(x), f64::from(y));
    }

    same_value_non_number(x, y)
}
