use std::ops::Deref;
use std::ops::DerefMut;

use crate::value::YololValue;

#[derive(Debug, Clone, Default)]
pub struct Field {
    name: String,
    value: YololValue,
}

impl Field {
    /// Get a reference to the field's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Set the field's name.
    pub fn set_name(&mut self, name: String) {
        self.name = name.to_lowercase();
    }
}

impl Deref for Field {
    type Target = YololValue;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Field {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
