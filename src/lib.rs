use std::collections::BTreeMap;
use std::ops::Deref;

use devices::chip::CodeRunner;
use devices::Device;
use field::Field;
use parser::YamlElement;

use crate::parser::YamlDocument;

pub mod devices;
pub mod field;
mod parser;
pub mod value;

#[derive(Debug)]
pub struct Networks<R: CodeRunner + Default> {
    networks: BTreeMap<String, Network<R>>,

    relays: Vec<(String, String)>,
}

impl<R: CodeRunner + Default> Networks<R> {
    pub fn deserialize(path: &str) -> Option<Self> {
        //let file = std::fs::read_to_string(path).ok()?;
        let yaml_document = YamlDocument::new(path)?;
        let yaml = &yaml_document[0];

        println!("Deserialize file version : {}", yaml["version"].as_str()?);

        let mut networks = BTreeMap::new();
        for network in yaml["networks"].as_vec()?.iter() {
            let name = network["name"].as_str()?;

            println!("deserialize network : {}", name);

            networks.insert(name.to_string(), Network::deserialize(network));
        }
        let mut relays = vec![];
        if let Some(_relays) = yaml["relays"].as_vec() {
            for relay in _relays {
                let relay = relay.as_map()?;
                let src = yaml_document.resolve_alias(&relay["src"])?["name"]
                    .as_str()?
                    .to_string();
                let dst = yaml_document.resolve_alias(&relay["dst"])?["name"]
                    .as_str()?
                    .to_string();

                relays.push((src, dst));
            }
        }
        Some(Self { networks, relays })
    }

    pub fn parse_all_chip_file(&mut self) {
        for network in self.networks.values_mut() {
            network.parse_all_chip_file();
        }
    }

    pub fn step(&mut self) {
        for network in self.networks.values_mut() {
            network.update_globals();
        }
        for network in self.networks.values_mut() {
            network.step();
        }
        for network in self.networks.values_mut() {
            network.update();
        }
        for (src, dst) in &self.relays {
            if let Some((_, src)) = self.networks.iter().find(|(s, _)| s == &src) {
                let src = src.globals().clone();
                if let Some((_, dst)) = self.networks.iter_mut().find(|(s, _)| s == &dst) {
                    dst.set_globals(src);
                }
            }
        }
    }

    pub fn print_globals(&self) {
        for (name, network) in &self.networks {
            println!("Globals of network : {}", name);
            network.print_globals();
        }
    }
}

#[derive(Debug)]
pub struct Network<R: CodeRunner + Default> {
    devices: Vec<Device<R>>,
    globals: Vec<Field>,
}

impl<R: CodeRunner + Default> Network<R> {
    pub fn parse_all_chip_file(&mut self) {
        for device in &mut self.devices {
            if let Device::Rack(rack) = device {
                rack.parse_all_chip_file()
            }
        }
    }

    pub fn step(&mut self) {
        for device in &mut self.devices {
            if let Device::Rack(rack) = device {
                rack.step();
            }
        }
    }

    pub fn update_globals(&mut self) {
        for device in &mut self.devices {
            if let Device::Rack(rack) = device {
                rack.update_globals(self.globals.clone());
            }
        }
    }

    pub fn update(&mut self) {
        let mut field = vec![];
        for device in &mut self.devices {
            if let Device::Rack(rack) = device {
                field.append(&mut rack.get_global());
            }
        }
        self.set_globals(field);
    }

    pub fn print_globals(&self) {
        for field in &self.globals {
            println!(":{} = {}", field.name(), **field)
        }
    }

    /// Get a reference to the network's globals.
    pub fn globals(&self) -> Vec<Field> {
        self.globals.clone()
    }

    /// Set the network's globals.
    pub fn set_globals(&mut self, globals: Vec<Field>) {
        for field in globals {
            let global = self
                .globals
                .iter_mut()
                .find(|i| i.name().to_lowercase() == field.name().to_lowercase());
            if let Some(global) = global {
                **global = field.deref().clone();
            }
        }
    }
}

impl<R: CodeRunner + Default> Network<R> {
    pub fn deserialize(yaml: &YamlElement) -> Self {
        let mut devices = vec![];
        if let Some(v) = yaml["devices"].as_vec() {
            for i in v.iter() {
                let device = Device::deserialize(i);
                if let Some(device) = device {
                    devices.push(device);
                }
            }
        }
        Self {
            devices,
            globals: vec![],
        }
    }
}
