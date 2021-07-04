mod int;
mod string;

use enum_dispatch::enum_dispatch;

pub use self::int::YololInt;
pub use self::string::YololString;

#[enum_dispatch]
trait ValueTrait {}

#[enum_dispatch(ValueTrait)]
pub enum YololValue {
    String(YololString),
    Int(YololInt),
}

impl From<String> for YololValue {
    fn from(v: String) -> Self {
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
