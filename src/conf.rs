use std::{fs::read_to_string, path::PathBuf};

use directories::BaseDirs;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub block_mode: bool,
    pub change_rate: f64,
    pub saturation: f64,
    pub value: f64,
    pub random: bool,
    pub chars: Option<String>,
}

const DEFAULT_CONFIG: Config = Config {
    block_mode: true,
    change_rate: 0.001,
    saturation: 1.0,
    value: 1.0,
    random: true,
    chars: None,
    // we cant use chars as a default because the string needs to be owned and cant be converted to owned in a const
};
const CONFIG_NAME: &str = "rustbow.toml";

/// Gets the config file and serializes it into a Config struct.
/// If the config file doesn't exist, it will be created with the default values.
/// # Returns
/// (Config, if its default)
pub fn get_config() -> (Config, bool) {
    let configpath = get_config_path();
    let mut is_default = false;
    if !configpath.exists() {
        write_default_config();
        is_default = true;
    }
    let configfile = read_to_string(configpath);
    let mut config: Config = DEFAULT_CONFIG;
    if configfile.is_err() {
        println!("Error: {}", configfile.err().unwrap());
        write_default_config();
    } else {
        config = load_from_string(configfile.unwrap());
    }
    (config, is_default)
}
/// Writes the default config to the config file.
fn write_default_config() {
    let configpath = get_config_path();
    let config = toml::to_string(&DEFAULT_CONFIG).unwrap();
    let conf_with_notes = 
        "# This is the default config file for rustbow.\n".to_owned() +
        config.as_str() +
        "\n# chars = \"\" # If you want to use custom characters, uncomment this line and add your characters here, also disable random.";
    let res = std::fs::write(configpath, conf_with_notes);
    if res.is_err() {
        println!("Error: {}", res.err().unwrap());
    }
}
/// Loads the config from a string. If the string is invalid, the default config will be returned.
fn load_from_string(configfile: String) -> Config {
    let config = toml::from_str(&configfile);
    if config.is_err() {
        println!("Error parsing configfile: {}", config.err().unwrap());
        DEFAULT_CONFIG
    } else {
        config.unwrap()
    }
}
/// Gets the path to the config file.
/// # Returns
/// The path to the config file.
pub fn get_config_path() -> PathBuf {
    let binding = BaseDirs::new().unwrap();
    let homepath = binding.config_dir();
    let configpath = homepath.join(CONFIG_NAME).as_path().to_owned();
    configpath
}
