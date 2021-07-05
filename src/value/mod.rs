mod int;
mod string;

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
pub enum YololValue {
    String(YololString),
    Int(YololInt),
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
