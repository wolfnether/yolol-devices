use yaml_rust::Yaml;

use crate::deserialize_field_name;
use crate::field::Field;
use crate::Network;
use crate::Networks;

#[derive(Debug)]
pub enum Chip<R: CodeRunner + Default> {
    None,
    Memory(MemoryChip),
    Yolol(YololChip<R>),
}

impl<R: CodeRunner + Default> Chip<R> {
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
pub struct YololChip<R: CodeRunner + Default> {
    chip_wait: Field,
    path: Option<String>,
    code: Option<R>,
}

impl<R: CodeRunner + Default> Default for Chip<R> {
    fn default() -> Self {
        Self::None
    }
}

pub trait CodeRunner: Default {
    fn parse(&mut self, path: &str);
    fn step(&mut self);
}

#[derive(Default)]
pub struct NoneRunner;
impl CodeRunner for NoneRunner {
    fn parse(&mut self, _: &str) {}
    fn step(&mut self) {}
}
