use std::collections::HashMap;

use serde::Deserialize;
use strum_macros::EnumString;

#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Layer {
    Top,
    Bottom,
}

#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Position {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Module {
    Battery,
    Clock,
    Cpu,
    Custom,
    Disk,
    Memory,
    #[strum(serialize = "pulseaudio")]
    PulseAudio,
    #[strum(serialize = "sway/workspaces")]
    SwayWorkspaces,
    Tray,
}

#[derive(Deserialize)]
pub struct Config {
    layer: Layer,
    position: Position,
    modules_left: Vec<String>,
    modules_center: Vec<String>,
    modules_right: Vec<String>,
    modules: HashMap<String, Module>,
}

// impl TryFrom<Value> for Config {
//     type Error = &'static str;

//     fn try_from(value: Value) -> Result<Self, Self::Error> {
//         Ok(Config {
//             layer: value["layer"],
//             position: value["position"],
//             modules_left: value["modules_left"],
//             modules_center: value["modules_center"],
//             modules_right: value["modules_right"],
//             // modules: HashMap<String, Module>,
//         })
//     }
// }
