
use std::{fs::read_to_string, path::PathBuf};

use directories::BaseDirs;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub block_mode: bool,
    pub change_rate: f64,
    pub saturation: f64,
    pub value: f64,
}

const DEFAULT_CONFIG: Config = Config {
    block_mode: true,
    change_rate: 0.001,
    saturation: 1.0,
    value: 1.0,
};
const CONFIG_NAME: &str = ".rustbow.toml";

pub fn get_config() -> Config {
    let configpath = get_config_path();
    let configfile = read_to_string(configpath);
    let mut config: Config = DEFAULT_CONFIG;
    if configfile.is_err() {
        println!("Error: {}", configfile.err().unwrap());
        write_default_config();
    } else {
        config = load_from_string(configfile.unwrap());
    }
    config
}

fn write_default_config(){
    let configpath = get_config_path();
    let config = toml::to_string(&DEFAULT_CONFIG).unwrap();
    let res = std::fs::write(configpath, config);
    if res.is_err() {
        println!("Error: {}", res.err().unwrap());
    }

}

fn load_from_string(configfile: String) -> Config {
    let config = toml::from_str(&configfile);
    if config.is_err() {
        println!("Error parsing configfile: {}", config.err().unwrap());
        DEFAULT_CONFIG
    } else {
        config.unwrap()
    }
}

fn get_config_path() -> PathBuf {
    let binding = BaseDirs::new().unwrap();
    let homepath = binding.home_dir();
    let configpath = homepath.join(CONFIG_NAME).as_path().to_owned();
    configpath
}