mod int;
mod string;

use std::convert::TryInto;
use std::ops::Add;
use std::ops::Sub;

use enum_dispatch::enum_dispatch;

pub use self::int::YololInt;
pub use self::string::YololString;

#[enum_dispatch]
trait ValueTrait {
    fn post_inc(&mut self) -> YololValue;
    fn pre_inc(&mut self) -> YololValue;
    fn post_dec(&mut self) -> YololValue;
    fn pre_dec(&mut self) -> YololValue;
}

#[enum_dispatch(ValueTrait)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum YololValue {
    String(YololString),
    Int(YololInt),
}

impl YololValue {
    fn is_string(&self) -> bool {
        match self {
            YololValue::String(_) => true,
            YololValue::Int(_) => false,
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

impl From<f32> for YololValue {
    fn from(v: f32) -> Self {
        Self::Int(v.into())
    }
}

impl From<f64> for YololValue {
    fn from(v: f64) -> Self {
        Self::Int(v.into())
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
                    let a: YololInt = self.try_into().unwrap();
                    a.into()
                }
            };
            let b: YololString = {
                if rhs.is_string() {
                    rhs.try_into().unwrap()
                } else {
                    let b: YololInt = rhs.try_into().unwrap();
                    b.into()
                }
            };
            (a + b).into()
        } else {
            let a: YololInt = self.try_into().unwrap();
            let b: YololInt = rhs.try_into().unwrap();
            (a + b).into()
        }
    }
}

impl Sub for YololValue {
    type Output = YololValue;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_string() || rhs.is_string() {
            let a: YololString = {
                if self.is_string() {
                    self.try_into().unwrap()
                } else {
                    let a: YololInt = self.try_into().unwrap();
                    a.into()
                }
            };
            let b: YololString = {
                if rhs.is_string() {
                    rhs.try_into().unwrap()
                } else {
                    let b: YololInt = rhs.try_into().unwrap();
                    b.into()
                }
            };
            (a - b).into()
        } else {
            let a: YololInt = self.try_into().unwrap();
            let b: YololInt = rhs.try_into().unwrap();
            (a - b).into()
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
