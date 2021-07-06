use yaml_rust::Yaml;

use crate::deserialize_field_name;
use crate::field::Field;

#[derive(Debug)]
pub enum Chip {
    None,
    Memory(MemoryChip),
    Yolol(YololChip),
}

impl Chip {
    pub fn new(chip_type: &str, yaml: &Yaml) -> Self {
        match chip_type {
            "memory_chip" => Self::Memory(MemoryChip::default()),
            "yolol_chip" => {
                let mut chip = YololChip::default();
                chip.path = yaml["script"].as_str().map(|s| s.to_string());
                deserialize_field_name!(chip, chip_wait, yaml);
                Self::Yolol(chip)
            }
            _ => Self::None,
        }
    }
}

#[derive(Debug, Default)]
pub struct MemoryChip {}

#[derive(Debug, Default)]
pub struct YololChip {
    chip_wait: Field,
    path: Option<String>,
}

impl Default for Chip {
    fn default() -> Self {
        Self::None
    }
}
