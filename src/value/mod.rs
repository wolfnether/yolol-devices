mod int;
mod string;

use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

use enum_dispatch::enum_dispatch;

pub use self::int::YololInt;
pub use self::string::YololString;

#[enum_dispatch]
pub trait ValueTrait {
    fn post_inc(&mut self) -> YololValue;
    fn pre_inc(&mut self) -> YololValue;
    fn post_dec(&mut self) -> Option<YololValue>;
    fn pre_dec(&mut self) -> Option<YololValue>;
    fn fac(&self) -> Option<YololValue>;
    fn abs(&self) -> Option<YololValue>;
    fn sqrt(&self) -> Option<YololValue>;
    fn sin(&self) -> Option<YololValue>;
    fn asin(&self) -> Option<YololValue>;
    fn cos(&self) -> Option<YololValue>;
    fn acos(&self) -> Option<YololValue>;
    fn tan(&self) -> Option<YololValue>;
    fn atan(&self) -> Option<YololValue>;
    fn pow(&self, e: YololValue) -> Option<YololValue>;
}

#[enum_dispatch(ValueTrait)]
#[derive(Debug, Clone)]
pub enum YololValue {
    String(YololString),
    Int(YololInt),
}

impl Default for YololValue {
    fn default() -> Self {
        YololValue::Int(YololInt::default())
    }
}

impl YololValue {
    fn is_string(&self) -> bool {
        match self {
            YololValue::String(_) => true,
            YololValue::Int(_) => false,
        }
    }
}

impl From<&YololValue> for bool {
    fn from(v: &YololValue) -> Self {
        match v {
            YololValue::String(v) => v.into(),
            YololValue::Int(v) => v.into(),
        }
    }
}

impl From<&str> for YololValue {
    fn from(v: &str) -> Self {
        Self::String(v.into())
    }
}

impl From<i64> for YololValue {
    fn from(v: i64) -> Self {
        Self::Int(v.into())
    }
}

impl From<f64> for YololValue {
    fn from(v: f64) -> Self {
        Self::Int(v.into())
    }
}

impl From<bool> for YololValue {
    fn from(v: bool) -> Self {
        Self::Int(v.into())
    }
}

impl TryFrom<&YololValue> for YololInt {
    type Error = ();

    fn try_from(value: &YololValue) -> Result<Self, Self::Error> {
        if let YololValue::Int(v) = value {
            Ok(*v)
        } else {
            Err(())
        }
    }
}

impl TryFrom<&YololValue> for YololString {
    type Error = ();

    fn try_from(value: &YololValue) -> Result<Self, Self::Error> {
        if let YololValue::String(v) = value {
            Ok(v.clone())
        } else {
            Err(())
        }
    }
}

impl YololValue {
    pub fn or(&self, rhs: &Self) -> Self {
        YololInt::from(self.into() || rhs.into()).into()
    }

    pub fn and(&self, rhs: &Self) -> Self {
        YololInt::from(self.into() && rhs.into()).into()
    }
}

impl Mul for YololValue {
    type Output = Option<YololValue>;

    fn mul(self, rhs: Self) -> Self::Output {
        if let YololValue::Int(lhs) = self {
            if let YololValue::Int(rhs) = rhs {
                return Some((lhs * rhs).into());
            }
        }
        None
    }
}

impl Div for YololValue {
    type Output = Option<YololValue>;

    fn div(self, rhs: Self) -> Self::Output {
        if let YololValue::Int(lhs) = self {
            if let YololValue::Int(rhs) = rhs {
                return Some((lhs / rhs)?.into());
            }
        }
        None
    }
}

impl Rem for YololValue {
    type Output = Option<YololValue>;

    fn rem(self, rhs: Self) -> Self::Output {
        if let YololValue::Int(lhs) = self {
            if let YololValue::Int(rhs) = rhs {
                if rhs != 0.into() {
                    return Some((lhs * rhs).into());
                }
            }
        }
        None
    }
}

impl Add for YololValue {
    type Output = YololValue;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_string() || rhs.is_string() {
            let a: YololString = {
                if self.is_string() {
                    self.try_into().unwrap()
                } else {
                    let a: &YololInt = &self.try_into().unwrap();
                    a.into()
                }
            };
            let b: YololString = {
                if rhs.is_string() {
                    rhs.try_into().unwrap()
                } else {
                    let b: &YololInt = &rhs.try_into().unwrap();
                    b.into()
                }
            };
            (a + b).into()
        } else {
            let a: YololInt = self.try_into().unwrap();
            let b: YololInt = rhs.try_into().unwrap();
            (&a + &b).into()
        }
    }
}

impl Sub for YololValue {
    type Output = Option<YololValue>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_string() || rhs.is_string() {
            let a: YololString = {
                if self.is_string() {
                    self.try_into().unwrap()
                } else {
                    let a: &YololInt = &self.try_into().unwrap();
                    a.into()
                }
            };
            let b: YololString = {
                if rhs.is_string() {
                    rhs.try_into().unwrap()
                } else {
                    let b: &YololInt = &rhs.try_into().unwrap();
                    b.into()
                }
            };
            Some((a - b)?.into())
        } else {
            let a: YololInt = self.try_into().unwrap();
            let b: YololInt = rhs.try_into().unwrap();
            Some((&a - &b).into())
        }
    }
}

impl PartialEq for YololValue {
    fn eq(&self, rhs: &Self) -> bool {
        if self.is_string() && rhs.is_string() {
            let a: YololString = self.clone().try_into().unwrap();
            let b: YololString = rhs.clone().try_into().unwrap();
            return a.eq(&b);
        } else if !self.is_string() && !rhs.is_string() {
            let a: YololInt = self.clone().try_into().unwrap();
            let b: YololInt = rhs.clone().try_into().unwrap();
            return a.eq(&b);
        }
        false
    }
}

impl PartialOrd for YololValue {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        if self.is_string() || rhs.is_string() {
            let a: YololString = {
                if self.is_string() {
                    self.clone().try_into().unwrap()
                } else {
                    let a: &YololInt = &self.try_into().unwrap();
                    a.into()
                }
            };
            let b: YololString = {
                if rhs.is_string() {
                    rhs.try_into().unwrap()
                } else {
                    let b: &YololInt = &rhs.clone().try_into().unwrap();
                    b.into()
                }
            };
            a.partial_cmp(&b)
        } else {
            let a: YololInt = self.clone().try_into().unwrap();
            let b: YololInt = rhs.clone().try_into().unwrap();
            a.partial_cmp(&b)
        }
    }
}

impl Display for YololValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YololValue::String(v) => write!(f, "{}", v),
            YololValue::Int(v) => write!(f, "{}", v),
        }
    }
}

#[test]
fn concat_test_1() {
    let a = 1;
    let b = 1;
    assert_eq!(
        YololValue::from(a + b),
        YololValue::from(a) + YololValue::from(b)
    )
}

#[test]
fn concat_test_2() {
    let a = "Hello";
    let b = 1;
    assert_eq!(
        YololValue::from("Hello1"),
        YololValue::from(a) + YololValue::from(b)
    )
}
