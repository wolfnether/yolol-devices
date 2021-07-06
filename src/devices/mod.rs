mod chip;
mod rack;
use concat_idents::concat_idents;
use convert_case::Case;
use convert_case::Casing;
use enum_dispatch::enum_dispatch;
use strum::EnumIter;
use strum::IntoEnumIterator;
use yaml_rust::Yaml;

pub use self::rack::Rack;
use crate::field::Field;
use crate::value::YololValue;

//thx https://github.com/martindevans/YololShipSystemSpec

#[enum_dispatch]
pub trait DeviceTrait {
    fn get_field(&self, field: &str) -> Option<&YololValue>;
    fn get_field_mut(&mut self, field: &str) -> Option<&mut YololValue>;
    fn get_device_name(&self) -> String;
    fn deserialize(self, yaml: &Yaml) -> Device;
}

#[enum_dispatch(DeviceTrait)]
#[derive(Debug, EnumIter)]
pub enum Device {
    Button,
    CargoBeam,
    CargoLockFrame,
    ChipSocket,
    FixedMount,
    FlightControlUnit,
    Generator,
    Hinge,
    InformationScreen,
    Lamp,
    Lever,
    MainFlightComputer,
    MiningLaser,
    ModularDisplay,
    Rack,
    RadioReceiver,
    RadioTransmitter,
    RailRelay,
    RailSensorStrip,
    RailTrigger,
    RangeFinder,
    Relay,
    Tank,
    Thruster,
    Turntable,
}

impl Device {
    pub fn deserialize(yaml: &Yaml) -> Option<Self> {
        let device_type = yaml.get_tag().expect("Need a type for deserializing");
        println!("trying to deserialize {}", device_type);
        let device = Device::iter().find(|i| i.get_device_name() == device_type)?;

        Some(device.deserialize(yaml))
    }
}

macro_rules! make_device {
    ($name:ident $(, $field:ident)+ $(,)?) => {
        #[derive(Debug,Default)]
        pub struct $name {
            $($field:Field,)+
        }

        impl $name{
            $(
                pub fn $field(&self)->&Field{
                    &self.$field
                }
                concat_idents!(fn_name = $field,_mut{
                    pub fn fn_name (&mut self)->&mut Field{
                        &mut self.$field
                    }
                });
            )+
        }

        impl DeviceTrait for $name{
            fn deserialize(mut self, yaml: &Yaml) -> Device {
                $(
                    let mut name = stringify!($field).to_case(Case::Pascal);
                    if let Some(_name) = yaml[name.as_str()].as_str(){
                        name = _name.to_string();
                    }
                    self.$field.set_name(name);
                )+
                self.into()
            }
            fn get_device_name(&self) -> String {
                stringify!($name).to_string().to_case(Case::Snake)
            }

            fn get_field(&self, field: &str) -> Option<&YololValue>{
                $(
                    if self.$field.name() == field {
                        return Some(&self.$field)
                    }
                )+
                None
            }

            fn get_field_mut(&mut self, field: &str) -> Option<&mut YololValue>{
                $(
                    if self.$field.name() == field {
                        return Some(&mut self.$field)
                    }
                )+
                None
            }
        }
    };
}

make_device!(
    Button,
    button_state,
    button_on_state_value,
    button_off_state_value,
    button_style
);
make_device!(CargoBeam, cargo_beam_on_state, cargo_beam_search_length);
make_device!(CargoLockFrame, cargo_frame_state);
make_device!(
    ChipSocket,
    button_state,
    button_on_state_value,
    button_off_state_value,
    button_style
);
make_device!(FixedMount, current_state, on_state, off_state, button_style);
make_device!(
    FlightControlUnit,
    fcu_mfc_io,
    fcu_general_multiplier,
    fcu_forward,
    fcu_backward,
    fcu_rotational_pitch,
    fcu_rotational_yaw,
    fcu_rotational_roll,
    fcu_up_down,
    fcu_right_left,
    fcu_fwd_bwd_pitch,
    fcu_fwd_bwd_yaw,
    fcu_fwd_bwd_roll
);
make_device!(
    Generator,
    fuel_chamber_fuel,
    fuel_chamber_max_fuel,
    fuel_chamber_unit_rate_limit,
    fuel_chamber_unit_rate,
    generator_unit_rate_limit,
    generator_unit_rate,
    stored_coolant,
    max_coolant,
    cooler_unit_rate_limit,
    cooler_unit_rate,
    socket_unit_rate_limit,
    socket_unit_rate
);
make_device!(
    Hinge,
    door_open_state,
    door_current_state,
    end_rotation,
    start_rotation,
    target_velocity
);
make_device!(InformationScreen, info_screen_content);
make_device!(
    Lamp,
    lamp_on,
    lamp_lumens,
    lamp_color_hue,
    lamp_color_saturation,
    lamp_color_value,
    lamp_range
);
make_device!(
    Lever,
    lever_state,
    lever_min_output,
    lever_max_output,
    lever_center_output,
    lever_center_dead_zone,
    lever_centering_speed,
    lever_binds_move_speed
);
make_device!(
    MainFlightComputer,
    fcu_mfc_io1,
    fcu_mfc_io2,
    thruster_power_level01,
    thruster_power_level02,
    thruster_power_level03,
    thruster_power_level04,
    thruster_power_level05,
    thruster_power_level06,
    thruster_power_level07,
    thruster_power_level08,
    thruster_power_level09,
    thruster_power_level10,
    thruster_power_level11,
    thruster_power_level12,
    thruster_power_level13,
    thruster_power_level14,
    thruster_power_level15,
    thruster_power_level16,
    thruster_power_level17,
    thruster_power_level18,
    thruster_power_level19,
    thruster_power_level20,
    thruster_power_level21,
    thruster_power_level22,
    thruster_power_level23,
    thruster_power_level24,
    thruster_power_level25,
    thruster_power_level26,
    thruster_power_level27,
    thruster_power_level28,
    thruster_power_level29,
    thruster_power_level30,
    thruster_power_level31,
    thruster_power_level32,
    thruster_power_level33,
    thruster_power_level34,
    thruster_power_level35,
    thruster_power_level36,
    thruster_power_level37,
    thruster_power_level38,
    thruster_power_level39,
    thruster_power_level40,
    thruster_power_level41,
    thruster_power_level42,
    thruster_power_level43,
    thruster_power_level44,
    thruster_power_level45,
    thruster_power_level46,
    thruster_power_level47,
    thruster_power_level48,
    thruster_power_level49,
    thruster_power_level50,
);
make_device!(MiningLaser, mining_laser_on, mining_laser_beam_length);
make_device!(ModularDisplay, panel_value);
make_device!(
    RadioReceiver,
    message,
    signal_strength,
    listen_angle,
    target_message,
    target_frequency,
    frequency,
    receiver_pitch,
    receiver_current_pitch,
    max_rotation,
    min_rotation,
    target_velocity
);
make_device!(
    RadioTransmitter,
    transmit_message,
    transmit_range,
    frequency
);
make_device!(RailRelay, is_enabled);
make_device!(
    RailSensorStrip,
    rail_sensor_output,
    rail_sensor_delta,
    rail_sensor_mover_filter
);
make_device!(
    RailTrigger,
    rail_trigger_output,
    rail_trigger_value,
    rail_trigger_read_mover
);
make_device!(
    RangeFinder,
    range_finder_on_state,
    range_finder_search_length,
    range_finder_distance
);
make_device!(Relay, is_enabled);
make_device!(
    Tank,
    gas_container_stored_resource,
    gas_container_max_resource,
    is_open_id,
    flow_id
);
make_device!(Thruster, thruster_state, thruster_current_thrust);
make_device!(
    Turntable,
    turret_rotation,
    turret_current_rotation,
    max_rotation,
    min_rotation,
    target_velocity
);
