use std::collections::BTreeMap;

use devices::Device;
use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

pub mod devices;
pub mod field;
pub mod value;

#[derive(Debug)]
struct Networks {
    networks: BTreeMap<String, Network>,

    relays: Vec<(String, String)>,
}

impl Networks {
    pub fn deserialize(path: &str) -> Option<Self> {
        let file = std::fs::read_to_string(path).ok()?;
        let yaml = yaml_rust::YamlLoader::load_from_str(&file).ok()?[0];
        println!("Deserialize file version : {}", yaml["version"].as_str()?);
        let mut networks = BTreeMap::new();
        for network in yaml["networks"].as_vec()?.iter() {
            let name = network["name"].as_str()?;

            println!("deserialize network : {}", name);

            networks.insert(name.to_string(), Network::deserialize(network));
        }
        let mut relays = vec![];
        for relay in yaml["relays"].as_vec()?.iter() {
            let src = get_value(relay["src"].as_hash()?, "name")?
                .as_str()?
                .to_string();
            let dst = get_value(relay["dst"].as_hash()?, "name")?
                .as_str()?
                .to_string();
            relays.push((src, dst));
        }
        Some(Self { networks, relays })
    }
}

fn get_value<'a>(hashmap: &'a Hash, key: &str) -> Option<&'a Yaml> {
    for (k, v) in hashmap {
        if k.as_str()? == key {
            return Some(v);
        }
    }
    None
}

#[derive(Debug)]
struct Network {
    devices: Vec<Device>,
}

impl Network {
    pub fn deserialize(yaml: &Yaml) -> Self {
        let mut devices = vec![];
        if let Some(v) = yaml["devices"].as_vec() {
            for i in v.iter() {
                let device = Device::deserialize(i);
                if let Some(device) = device {
                    devices.push(device);
                }
            }
        }
        Self { devices }
    }
}
