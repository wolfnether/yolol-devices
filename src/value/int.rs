use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Not;
use std::ops::Rem;
use std::ops::RemAssign;
use std::ops::Sub;
use std::ops::SubAssign;

use super::ValueTrait;
use super::YololValue;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct YololInt(i64);

macro_rules! yolol_int_function {
     ($n:ident $(, $nl:ident)+ $(,)?) => {
        pub fn $n(&self) -> Self {
                Self::from((self.0 as f64 / 1000.).$n())
        }
        $(
            pub fn $nl(&self) -> Self {
                Self::from((self.0 as f64 / 1000.).$nl())
            }
        )*
    }
}

impl YololInt {
    pub fn abs(&self) -> Self {
        YololInt(self.0.abs())
    }

    pub fn pow(&self, exp: YololInt) -> Self {
        Self::from((self.0 as f64 / 1000.).powf(exp.0 as f64 / 1000.))
    }

    pub fn fac(&self) -> Self {
        Self::from((1..=self.0 / 1000).fold(1, |r, i| r * i))
    }

    yolol_int_function!(sqrt, sin, cos, tan, asin, acos, atan);
}

impl ValueTrait for YololInt {
    fn post_inc(&mut self) -> YololValue {
        *self += *self + 1.into();
        self.clone().into()
    }

    fn pre_inc(&mut self) -> YololValue {
        *self += *self + 1.into();
        self.clone().into()
    }

    fn post_dec(&mut self) -> YololValue {
        *self += *self - 1.into();
        self.clone().into()
    }

    fn pre_dec(&mut self) -> YololValue {
        *self += *self - 1.into();
        self.clone().into()
    }
}

impl Deref for YololInt {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for YololInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<i64> for YololInt {
    fn from(v: i64) -> Self {
        Self(v * 1000)
    }
}

impl From<f32> for YololInt {
    fn from(v: f32) -> Self {
        Self((v * 1000.) as i64)
    }
}

impl From<f64> for YololInt {
    fn from(v: f64) -> Self {
        Self((v * 1000.).floor() as i64)
    }
}

impl From<bool> for YololInt {
    fn from(v: bool) -> Self {
        Self(1000 * v as i64)
    }
}

impl Add for YololInt {
    type Output = YololInt;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for YololInt {
    type Output = YololInt;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for YololInt {
    type Output = YololInt;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from((self.0 as f64 / 1000.) * (rhs.0 as f64 / 1000.))
    }
}

impl Div for YololInt {
    type Output = YololInt;
    fn div(self, rhs: Self) -> Self::Output {
        Self::from((self.0 as f64 / 1000.) / (rhs.0 as f64 / 1000.))
    }
}

impl Rem for YololInt {
    type Output = YololInt;
    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl Not for YololInt {
    type Output = YololInt;
    fn not(self) -> Self::Output {
        Self::from(self.0 == 0)
    }
}

impl AddAssign for YololInt {
    fn add_assign(&mut self, rhs: YololInt) {
        self.0 += rhs.0
    }
}

impl SubAssign for YololInt {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl MulAssign for YololInt {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for YololInt {
    fn div_assign(&mut self, rhs: YololInt) {
        *self = *self / rhs;
    }
}

impl RemAssign for YololInt {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl Display for YololInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("{}", (self.0 as f32) / 1000.))
    }
}

macro_rules! yolol_int_test {
    ($n:ident,$a:expr,$op:tt,$b:expr) => {
        #[test]
        fn $n(){
            let a = YololInt::from($a) $op YololInt::from($b);
            let b = YololInt::from($a $op $b);
            if a != b {
                panic!("{} {} {} = {:?} != {:?}",$a,stringify!($op),$b, a,b)
            }
        }
    };
    ($n:ident, _, $op:tt,$b:expr) => {        #[test]
        fn $n(){
            let a = $op YololInt::from($b);
            let b =YololInt::from($op $b);
            if a != b {
                panic!("{} {} = {:?} != {:?}",stringify!($op),$b, a,b)
            }
        }
    };
}
yolol_int_test!(test_add_1, 1,+,1);
yolol_int_test!(test_add_2, -1,+,1);
yolol_int_test!(test_add_3, 1,+,-1);
yolol_int_test!(test_add_4, -1,+,-1);
yolol_int_test!(test_sub_1 , 1, -, 1);
yolol_int_test!( test_sub_2,-1, -, 1);
yolol_int_test!( test_sub_3, 1, - ,-1);
yolol_int_test!( test_sub_4, -1, -, -1);
yolol_int_test!( test_mul_1, 1, *, 1);
yolol_int_test!( test_mul_2, -1, *, 1);
yolol_int_test!( test_mul_3, 1, *, -1);
yolol_int_test!( test_mul_4, -1, *, -1);
yolol_int_test!( test_mul_5, 1000., *, 0.01);
yolol_int_test!( test_mul_6, 0.01, * ,1000.);
yolol_int_test!( test_div_1, 1, /, 1);
yolol_int_test!( test_div_2, -1, /, 1);
yolol_int_test!( test_div_3,1, /, -1);
yolol_int_test!( test_div_4,-1, /, -1);
yolol_int_test!( test_div_5,1000., /, 0.01);
yolol_int_test!(test_div_6,0.01,/,1000.);
yolol_int_test!( test_mod_1,10, %, 2);
yolol_int_test!( test_mod_2 ,10., % ,3.14);
yolol_int_test!(test_not_1, _, !, false);
yolol_int_test!(test_not_2, _, !, true);
