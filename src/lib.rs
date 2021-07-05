use std::collections::BTreeMap;

use devices::Device;
use yaml_rust::Yaml;
use yaml_rust::yaml::Hash;

pub mod devices;
pub mod field;
pub mod value;

#[derive(Debug)]
struct Networks {
    networks: BTreeMap<String, Network>,

    relays: Vec<(String, String)>,
}

impl Networks {
    pub fn deserialize(yaml: &Yaml) -> Option<Self> {
        println!("Deserialize file version : {}", yaml["version"].as_str()?);
        let mut networks = BTreeMap::new();
        for network in yaml["networks"].as_vec()?.iter() {
            let name = network["name"].as_str()?;

            println!("deserialize network : {}", name);

            let network = Network::deserialize(network);
            if let Some(network) = network{
                networks.insert(
                    name.to_string(),
                    network
                );
            } else {
                println!("ignored maybe empty or an error happend");
            }
        }
        let mut relays = vec![];
        for relay in yaml["relays"].as_vec()?.iter(){
            let src = get_value(relay["src"].as_hash()?, "name")?.as_str()?.to_string();
            let dst = get_value(relay["dst"].as_hash()?, "name")?.as_str()?.to_string();
            relays.push((src,dst));
        }
        Some(Self { networks, relays })
    }
}

fn get_value<'a>(hashmap:&'a Hash,key: &str) -> Option<&'a Yaml>{
    for (k,v) in hashmap{
        println!("{:?} {:?}",k,v);
        if k.as_str()? == key{
            return Some(v)
        }
    }
    None
}

#[derive(Debug)]
struct Network {
    devices: Vec<Device>,
}

impl Network {
    pub fn deserialize(yaml: &Yaml) -> Option<Self> {
        let mut devices = vec![];
        for i in yaml["devices"].as_vec()?.iter() {
            devices.push(Device::deserialize(i)?);
        }
        Some(Self { devices })
    }
}
