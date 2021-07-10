use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Not;
use std::ops::Rem;
use std::ops::Sub;

use super::ValueTrait;
use super::YololValue;

#[derive(Clone, Copy, Debug, Default)]
pub struct YololInt(i64);

impl YololInt {
    pub fn new_raw(v: i64) -> Self {
        Self(v)
    }
}

impl PartialEq for YololInt {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialOrd for YololInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl ValueTrait for YololInt {
    fn post_inc(&mut self) -> YololValue {
        *self = &*self + &1.into();
        (*self).into()
    }
    fn pre_inc(&mut self) -> YololValue {
        let o = *self;
        *self = &*self + &1.into();
        o.into()
    }

    fn post_dec(&mut self) -> Option<YololValue> {
        *self = &*self - &1.into();
        Some((*self).into())
    }

    fn pre_dec(&mut self) -> Option<YololValue> {
        let o = *self;
        *self = &*self - &1.into();
        Some(o.into())
    }

    fn fac(&self) -> Option<YololValue> {
        let b = self.into();
        let mut product: i64 = 1;
        if b < 0 {
            return Some(Self(-9223372036854775808).into());
        } else if b > 1 {
            for i in 1..=b {
                product = product.wrapping_mul(i);
            }
        } else {
            return Some(1.into());
        }
        Some(product.into())
    }

    fn abs(&self) -> Option<YololValue> {
        if self.0 == -9223372036854775808 {
            return Some(Self(-9223372036854775808).into());
        }
        let f: f64 = self.into();
        Some(f.abs().into())
    }

    fn sqrt(&self) -> Option<YololValue> {
        let f = self.0;
        if !(0..9223372036854775000).contains(&f) {
            return Some(Self(-9223372036854775808).into());
        }
        let v = (f as f64).sqrt() / 31.6227766017;
        let k = ((v % 0.001) * 10000.).round() as i64 / 10;
        Some(Self((v * 1000.).trunc() as i64 + k).into())
    }

    fn sin(&self) -> Option<YololValue> {
        let f: f64 = self.into();
        Some(f.to_radians().sin().into())
    }

    fn asin(&self) -> Option<YololValue> {
        let f: f64 = self.into();
        if !(-1. ..=1.).contains(&f) {
            return Some(Self(-9223372036854775808).into());
        }
        Some(f.asin().to_degrees().into())
    }

    fn cos(&self) -> Option<YololValue> {
        let f: f64 = self.into();
        Some(f.to_radians().cos().into())
    }

    fn acos(&self) -> Option<YololValue> {
        let f: f64 = self.into();
        if !(-1. ..=1.).contains(&f) {
            return Some(Self(-9223372036854775808).into());
        }
        Some(f.acos().to_degrees().into())
    }

    fn tan(&self) -> Option<YololValue> {
        let sin = &self.sin()?;
        let cos = &self.cos()?;
        if cos == &0.into() {
            Some(Self(-22877332428).into())
        } else {
            sin / cos
        }
    }

    fn atan(&self) -> Option<YololValue> {
        let f: f64 = self.into();
        Some(f.atan().to_degrees().into())
    }

    fn pow(&self, e: &YololValue) -> Option<YololValue> {
        match e {
            YololValue::String(_) => None,
            YololValue::Int(v) => {
                let r = (self.0 as f64 / 1000.).powf(v.0 as f64 / 1000.);
                if !r.is_normal() || r >= 9223372036854775.807 || r <= -9223372036854775.808 {
                    return Some(Self(-9223372036854775808).into());
                }
                Some(r.into())
            }
        }
    }
}

impl Add for &YololInt {
    type Output = YololInt;
    fn add(self, rhs: Self) -> Self::Output {
        YololInt(self.0.saturating_add(rhs.0))
    }
}

impl Sub for &YololInt {
    type Output = YololInt;
    fn sub(self, rhs: Self) -> Self::Output {
        YololInt(self.0.saturating_sub(rhs.0))
    }
}

impl Mul for &YololInt {
    type Output = YololInt;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut r = self.0;
        r = r.wrapping_mul(rhs.0);
        YololInt((r / 1000) as i64)
    }
}

impl Div for &YololInt {
    type Output = Option<YololInt>;
    fn div(self, rhs: Self) -> Self::Output {
        if rhs == &0.into() {
            return None;
        }
        Some(YololInt::from(
            (self.0 as f64 / 1000.) / (rhs.0 as f64 / 1000.),
        ))
    }
}

impl Rem for &YololInt {
    type Output = Option<YololInt>;
    fn rem(self, rhs: Self) -> Self::Output {
        if rhs == &0.into() {
            return None;
        }
        Some(YololInt(self.0 % rhs.0))
    }
}

impl Not for &YololInt {
    type Output = YololInt;
    fn not(self) -> Self::Output {
        (self.0 == 0).into()
    }
}

impl Display for YololInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let d = self.0 / 1000;
        let mut r = (self.0 - d * 1000).abs();
        if r < 10 {
            r *= 100;
        } else if r < 100 {
            r *= 10;
        }
        if r == 0 {
            f.write_fmt(format_args!("{}", d))
        } else {
            f.write_fmt(format_args!("{}.{}", d, r))
        }
    }
}

impl From<&YololInt> for bool {
    fn from(v: &YololInt) -> Self {
        v.0 != 0
    }
}

impl From<&YololInt> for i64 {
    fn from(b: &YololInt) -> Self {
        b.0 / 1000
    }
}

impl From<&YololInt> for usize {
    fn from(b: &YololInt) -> Self {
        b.0 as usize / 1000
    }
}

impl From<&YololInt> for f64 {
    fn from(v: &YololInt) -> Self {
        v.0 as f64 / 1000.
    }
}

impl From<bool> for YololInt {
    fn from(v: bool) -> Self {
        Self(v as i64 * 1000)
    }
}

impl From<i64> for YololInt {
    fn from(v: i64) -> Self {
        Self(v * 1000)
    }
}

impl From<f64> for YololInt {
    fn from(v: f64) -> Self {
        Self((v * 1000.).round() as i64)
    }
}
