use yaml_rust::Yaml;

use crate::deserialize_field_name;
use crate::field::Field;

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

    pub fn load(&mut self){
        if let Self::Yolol(chip) = self{
            if let Some(path) = chip.path.clone(){
                chip.runner = R::default().parse(&path);
            }
        }
    }

    pub fn step(&mut self){
        if let Self::Yolol(chip) = self{
            if chip.chip_wait.clone().into() {
                if let Some(runner) = &mut chip.runner{
                    runner.step()
                }
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct MemoryChip {}

#[derive(Debug, Default)]
pub struct YololChip<R: CodeRunner + Default> {
    chip_wait: Field,
    path: Option<String>,
    runner: Option<R>,
}

impl<R: CodeRunner + Default> Default for Chip<R> {
    fn default() -> Self {
        Self::None
    }
}

pub trait CodeRunner: Default {
    fn parse(&mut self, path: &str) -> Option<Self>;
    fn step(&mut self);
}

#[derive(Default)]
pub struct NoneRunner;
impl CodeRunner for NoneRunner {
    fn parse(&mut self, _: &str) -> Option<Self> {None}
    fn step(&mut self) {}
}
