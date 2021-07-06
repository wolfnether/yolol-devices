use super::chip::Chip;
use super::DeviceTrait;
use crate::field::Field;

#[derive(Debug, Default)]
pub struct Rack {
    current_state: Field,
    on_state: Field,
    off_state: Field,
    button: Field,

    module: RackModule,
}

#[derive(Debug)]
enum RackModule {
    Core(Chip, Chip, Chip),
    Socket(Chip, Chip),
    Reader(Chip),
}

impl Default for RackModule {
    fn default() -> Self {
        Self::Reader(Chip::default())
    }
}

impl DeviceTrait for Rack {
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

    fn deserialize(mut self, yaml: &yaml_rust::Yaml) -> super::Device {
        let mut name = "CurrentState";
        if let Some(_name) = yaml[name].as_str() {
            name = _name;
        }
        self.current_state.set_name(name.into());
        let mut name = "OnState";
        if let Some(_name) = yaml[name].as_str() {
            name = _name;
        }
        self.on_state.set_name(name.into());
        let mut name = "OffState";
        if let Some(_name) = yaml[name].as_str() {
            name = _name;
        }
        self.off_state.set_name(name.into());
        let mut name = "ButtonStyle";
        if let Some(_name) = yaml[name].as_str() {
            name = _name;
        }
        self.button.set_name(name.into());

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
