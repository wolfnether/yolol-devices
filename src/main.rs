use yolol_devices::devices::chip::NoneRunner;
use yolol_devices::Networks;

fn main() {
    println!(">>> {:?}", Networks::<NoneRunner>::deserialize("all.yaml"))
}
