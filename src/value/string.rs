use std::ops::Deref;
use std::ops::DerefMut;

pub struct YololString(String);

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

impl From<String> for YololString {
    fn from(v: String) -> Self {
        Self(v)
    }
}
