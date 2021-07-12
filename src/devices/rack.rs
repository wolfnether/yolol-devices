use std::ops::Deref;

use super::chip::Chip;
use super::chip::CodeRunner;
use super::DeviceTrait;
use crate::deserialize_field_name;
use crate::deserializer::Deserializer;
use crate::field::Field;

#[derive(Debug, Default)]
pub struct Rack<R: CodeRunner + Default> {
    current_state: Field,
    on_state: Field,
    off_state: Field,
    button: Field,

    module: RackModule<R>,
}

impl<R: CodeRunner + Default> Rack<R> {
    pub fn parse_all_chip_file(&mut self) {
        match &mut self.module {
            RackModule::Core(a, b, c) => {
                a.load();
                b.load();
                c.load()
            }
            RackModule::Socket(a, b) => {
                a.load();
                b.load()
            }
            RackModule::Reader(a) => a.load(),
        }
    }

    pub fn step(&mut self) {
        match &mut self.module {
            RackModule::Core(a, b, c) => {
                a.step();
                b.step();
                c.step()
            }
            RackModule::Socket(a, b) => {
                a.step();
                b.step()
            }
            RackModule::Reader(a) => a.step(),
        }
    }

    pub fn update_globals(&mut self, globals: Vec<Field>) {
        match &mut self.module {
            RackModule::Core(a, b, c) => {
                a.update_globals(globals.clone());
                b.update_globals(globals.clone());
                c.update_globals(globals)
            }
            RackModule::Socket(a, b) => {
                a.update_globals(globals.clone());
                b.update_globals(globals)
            }
            RackModule::Reader(a) => a.update_globals(globals),
        }
    }

    pub fn get_global(&self) -> Vec<Field> {
        match &self.module {
            RackModule::Core(a, b, c) => {
                let mut a = a.get_global();
                a.append(&mut b.get_global());
                a.append(&mut c.get_global());
                a
            }
            RackModule::Socket(a, b) => {
                let mut a = a.get_global();
                a.append(&mut b.get_global());
                a
            }
            RackModule::Reader(a) => a.get_global(),
        }
    }
}

#[derive(Debug)]
enum RackModule<R: CodeRunner + Default> {
    Core(Chip<R>, Chip<R>, Chip<R>),
    Socket(Chip<R>, Chip<R>),
    Reader(Chip<R>),
}

impl<R: CodeRunner + Default> Default for RackModule<R> {
    fn default() -> Self {
        Self::Reader(Chip::default())
    }
}

impl<R: CodeRunner + Default> DeviceTrait for Rack<R> {
    fn get_field(&self, field: &str) -> Option<&crate::value::YololValue> {
        if self.current_state.name() == field {
            return Some(self.current_state.deref());
        }
        if self.on_state.name() == field {
            return Some(&self.on_state);
        }
        if self.off_state.name() == field {
            return Some(&self.current_state);
        }
        if self.button.name() == field {
            return Some(&self.button);
        }
        None
    }

    fn get_field_mut(&mut self, field: &str) -> Option<&mut crate::value::YololValue> {
        if self.current_state.name() == field {
            return Some(&mut self.current_state);
        }
        if self.on_state.name() == field {
            return Some(&mut self.on_state);
        }
        if self.off_state.name() == field {
            return Some(&mut self.current_state);
        }
        if self.button.name() == field {
            return Some(&mut self.button);
        }
        None
    }

    fn get_device_name(&self) -> String {
        "rack".to_string()
    }

    fn deserialize<D>(&mut self, deserializer: &D)
    where
        D: Deserializer<D, Output = D>,
    {
        deserialize_field_name!(self, current_state, deserializer);
        deserialize_field_name!(self, on_state, deserializer);
        deserialize_field_name!(self, off_state, deserializer);
        deserialize_field_name!(self, button, deserializer);

        if let Some(tag) = deserializer["module".to_string()].get_type() {
            let modules = &deserializer["module".to_string()];
            match tag.as_str() {
                "!socker_core" => {
                    let chip1 = modules["slot1".to_string()]
                        .get_type()
                        .map(|tag| Chip::deserialize(tag, &modules["slot1".to_string()]))
                        .unwrap_or(Chip::None);
                    let chip2 = modules["slot2".to_string()]
                        .get_type()
                        .map(|tag| Chip::deserialize(tag, &modules["slot2".to_string()]))
                        .unwrap_or(Chip::None);
                    let rack_module = RackModule::Socket(chip1, chip2);
                    self.module = rack_module;
                }
                "!chip_core" => {
                    let chip1 = modules["slot1".to_string()]
                        .get_type()
                        .map(|tag| Chip::deserialize(tag, &modules["slot1".to_string()]))
                        .unwrap_or(Chip::None);
                    let chip2 = modules["slot2".to_string()]
                        .get_type()
                        .map(|tag| Chip::deserialize(tag, &modules["slot2".to_string()]))
                        .unwrap_or(Chip::None);
                    let chip3 = modules["slot3".to_string()]
                        .get_type()
                        .map(|tag| Chip::deserialize(tag, &modules["slot3".to_string()]))
                        .unwrap_or(Chip::None);
                    let rack_module = RackModule::Core(chip1, chip2, chip3);
                    self.module = rack_module;
                }
                "!chip_reader" => {
                    let chip1 = modules["slot1".to_string()]
                        .get_type()
                        .map(|tag| Chip::deserialize(tag, &modules["slot1".to_string()]))
                        .unwrap_or(Chip::None);
                    let rack_module = RackModule::Reader(chip1);
                    self.module = rack_module;
                }
                _ => (),
            }
        }
    }
}
