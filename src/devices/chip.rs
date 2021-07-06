use crate::field::Field;

#[derive(Debug)]
pub enum Chip {
    None,
    Memory(MemoryChip),
    Yolol(YololChip),
}

impl Chip {
    pub fn new(chip_type: &str) -> Self {
        match chip_type {
            "memory_chip" => Self::Memory(MemoryChip::default()),
            "yolol_chip" => Self::Yolol(YololChip::default()),
            _ => Self::None,
        }
    }
}

#[derive(Debug, Default)]
pub struct MemoryChip {}

#[derive(Debug, Default)]
pub struct YololChip {
    chip_wait: Field,
    //TODO something
}

impl Default for Chip {
    fn default() -> Self {
        Self::None
    }
}
