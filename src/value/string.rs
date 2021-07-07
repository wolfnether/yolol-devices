use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Sub;
use std::ops::SubAssign;

use super::ValueTrait;
use super::YololInt;
use super::YololValue;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
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

    fn post_dec(&mut self) -> YololValue {
        if self.0.len() == 0 {
            return self.clone().into();
        }
        let org = self.clone();
        *self = self.0[0..self.0.len() - 1].into();
        org.into()
    }

    fn pre_dec(&mut self) -> YololValue {
        if self.0.len() == 0 {
            return self.clone().into();
        }
        *self = self.0[0..self.0.len() - 1].into();
        self.clone().into()
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

impl From<YololInt> for YololString {
    fn from(int: YololInt) -> Self {
        YololString(format!("{}", *int as f64 / 1000.))
    }
}

impl Add for YololString {
    type Output = YololString;

    fn add(self, rhs: Self) -> Self::Output {
        Self(format!("{}{}", self.0, rhs.0))
    }
}

impl Sub for YololString {
    type Output = YololString;

    fn sub(self, rhs: Self) -> Self::Output {
        if rhs.0.len() > self.0.len() {
            return self.clone();
        } else if rhs.0 == self.0 {
            return Self("".into());
        } else {
            for i in 0..(self.0.len() - rhs.0.len()) {
                let s = self.0.len() - i - rhs.0.len();
                let e = self.0.len() - i;
                if rhs.0 == self.0[s..e] {
                    return Self(format!("{}{}", &self.0[0..s], &self.0[e..self.0.len()]));
                }
            }
        }
        return self.clone();
    }
}

impl AddAssign for YololString {
    fn add_assign(&mut self, rhs: YololString) {
        *self = self.clone() + rhs;
    }
}

impl SubAssign for YololString {
    fn sub_assign(&mut self, rhs: YololString) {
        *self = self.clone() - rhs
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
        YololString::from(a) - YololString::from(b)
    )
}

#[test]
fn remove_test_2() {
    let a = "Hello world";
    let b = " world";
    assert_eq!(
        YololString::from("Hello"),
        YololString::from(a) - YololString::from(b)
    )
}

#[test]
fn remove_test_3() {
    let b = "Hello world";
    let a = " world";
    assert_eq!(
        YololString::from(" world"),
        YololString::from(a) - YololString::from(b)
    )
}

#[test]
fn remove_test_4() {
    let a = "Hello world Hello world";
    let b = " world";
    assert_eq!(
        YololString::from("Hello world Hello"),
        YololString::from(a) - YololString::from(b)
    )
}
