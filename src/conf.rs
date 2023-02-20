use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub block_mode: bool,
    pub change_rate: f64,
    pub saturation: f64,
    pub value: f64,
}