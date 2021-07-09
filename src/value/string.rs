use std::fmt::Display;
use std::ops::Add;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Sub;

use super::ValueTrait;
use super::YololInt;
use super::YololValue;

#[derive(Debug, Clone, Default)]
pub struct YololString(String);

impl ValueTrait for YololString {
    fn post_inc(&mut self) -> YololValue {
        let org = self.clone();
        *self = self.clone() + " ".into();
        org.into()
    }
    fn pre_inc(&mut self) -> YololValue {
        *self = self.clone() + " ".into();
        self.clone().into()
    }

    fn post_dec(&mut self) -> Option<YololValue> {
        if self.0.is_empty() {
            return None;
        }
        let org = self.clone();
        *self = self.0[0..self.0.len() - 1].into();
        Some(org.into())
    }

    fn pre_dec(&mut self) -> Option<YololValue> {
        if self.0.is_empty() {
            return None;
        }
        *self = self.0[0..self.0.len() - 1].into();
        Some(self.clone().into())
    }

    fn fac(&self) -> Option<YololValue> {
        //runtime error
        None
    }

    fn abs(&self) -> Option<YololValue> {
        None
    }

    fn sqrt(&self) -> Option<YololValue> {
        None
    }

    fn sin(&self) -> Option<YololValue> {
        None
    }

    fn asin(&self) -> Option<YololValue> {
        None
    }

    fn cos(&self) -> Option<YololValue> {
        None
    }

    fn acos(&self) -> Option<YololValue> {
        None
    }

    fn tan(&self) -> Option<YololValue> {
        None
    }

    fn atan(&self) -> Option<YololValue> {
        None
    }

    fn pow(&self, _: &YololValue) -> Option<YololValue> {
        None
    }
}

impl PartialEq for YololString {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialOrd for YololString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl From<&YololString> for bool {
    fn from(_: &YololString) -> Self {
        false
    }
}

impl Deref for YololString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for YololString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&str> for YololString {
    fn from(v: &str) -> Self {
        Self(v.to_string())
    }
}

impl From<&YololInt> for YololString {
    fn from(int: &YololInt) -> Self {
        let f: f64 = int.into();
        YololString(format!("{}", f))
    }
}

impl Add for YololString {
    type Output = YololString;

    fn add(self, rhs: Self) -> Self::Output {
        Self(format!("{}{}", self.0, rhs.0))
    }
}

impl Sub for YololString {
    type Output = Option<YololString>;

    fn sub(self, rhs: Self) -> Self::Output {
        if rhs.0.len() > self.0.len() {
            return None;
        } else if rhs.0 == self.0 {
            return Some(Self("".into()));
        } else {
            for i in 0..(self.0.len() - rhs.0.len()) {
                let s = self.0.len() - i - rhs.0.len();
                let e = self.0.len() - i;
                if rhs.0 == self.0[s..e] {
                    return Some(Self(format!(
                        "{}{}",
                        &self.0[0..s],
                        &self.0[e..self.0.len()]
                    )));
                }
            }
        }
        Some(self)
    }
}

impl Display for YololString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[test]
fn concat_test() {
    let a = "Hello";
    let b = " world";
    assert_eq!(
        YololString::from("Hello world"),
        YololString::from(a) + YololString::from(b)
    )
}

#[test]
fn remove_test_1() {
    let a = "Hello";
    let b = " world";
    assert_eq!(
        YololString::from("Hello"),
        (YololString::from(a) - YololString::from(b)).unwrap()
    )
}

#[test]
fn remove_test_2() {
    let a = "Hello world";
    let b = " world";
    assert_eq!(
        YololString::from("Hello"),
        (YololString::from(a) - YololString::from(b)).unwrap()
    )
}

#[test]
fn remove_test_3() {
    let b = "Hello world";
    let a = " world";
    assert_eq!(
        YololString::from(" world"),
        (YololString::from(a) - YololString::from(b)).unwrap()
    )
}

#[test]
fn remove_test_4() {
    let a = "Hello world Hello world";
    let b = " world";
    assert_eq!(
        YololString::from("Hello world Hello"),
        (YololString::from(a) - YololString::from(b)).unwrap()
    )
}
