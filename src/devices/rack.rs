use super::chip::Chip;
use super::chip::CodeRunner;
use super::DeviceTrait;
use crate::deserialize_field_name;
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

impl<R: CodeRunner + Default> DeviceTrait<R> for Rack<R> {
    fn get_field(&self, field: &str) -> Option<&crate::value::YololValue> {
        if self.current_state.name() == field {
            return Some(&self.current_state);
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

    fn deserialize(mut self, yaml: &yaml_rust::Yaml) -> super::Device<R> {
        deserialize_field_name!(self, current_state, yaml);
        deserialize_field_name!(self, on_state, yaml);
        deserialize_field_name!(self, off_state, yaml);
        deserialize_field_name!(self, button, yaml);

        if let Some(tag) = yaml["module"].get_tag() {
            let modules = &yaml["module"];
            match tag {
                "socker_core" => {
                    let chip1 = modules["slot1"]
                        .get_tag()
                        .map(|tag| Chip::new(tag, &modules["slot1"]))
                        .unwrap_or(Chip::None);
                    let chip2 = modules["slot2"]
                        .get_tag()
                        .map(|tag| Chip::new(tag, &modules["slot2"]))
                        .unwrap_or(Chip::None);
                    let rack_module = RackModule::Socket(chip1, chip2);
                    self.module = rack_module;
                }
                "chip_core" => {
                    let chip1 = modules["slot1"]
                        .get_tag()
                        .map(|tag| Chip::new(tag, &modules["slot1"]))
                        .unwrap_or(Chip::None);
                    let chip2 = modules["slot2"]
                        .get_tag()
                        .map(|tag| Chip::new(tag, &modules["slot2"]))
                        .unwrap_or(Chip::None);
                    let chip3 = modules["slot3"]
                        .get_tag()
                        .map(|tag| Chip::new(tag, &modules["slot3"]))
                        .unwrap_or(Chip::None);
                    let rack_module = RackModule::Core(chip1, chip2, chip3);
                    self.module = rack_module;
                }
                "chip_reader" => {
                    let chip1 = modules["slot1"]
                        .get_tag()
                        .map(|tag| Chip::new(tag, &modules["slot1"]))
                        .unwrap_or(Chip::None);
                    let rack_module = RackModule::Reader(chip1);
                    self.module = rack_module;
                }
                _ => (),
            }
        }

        self.into()
    }
}
