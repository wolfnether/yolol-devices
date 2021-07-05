use std::ops::Deref;
use std::ops::DerefMut;

use yaml_rust::Yaml;

use crate::value::YololValue;

#[derive(Debug, Default)]
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
        self.name = name;
    }

    pub fn deserialize(&self, yaml: &Yaml) -> Option<()> {
        if let Some(v) = yaml.as_f64(){
            panic!("f {}", v);
        } else if let Some(v) = yaml.as_str(){
            panic!("s {}", v);
        }
        Some(())
    }
}

impl Deref for Field {
    type Target = YololValue;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Field {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.value
    }
}
