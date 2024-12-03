use std::{env, fs};
use bt_logger::log_verbose;

pub fn get_toml(env_variable: &str, or_file_name: &str) -> toml::Value{
    let cfg_file: String;
    let error_msg = format!("Unable to read config TOML file from env variable: {} or default file: {}.", &env_variable, &or_file_name );

    match env::var(env_variable) {
        Ok(value) => cfg_file = value,
        Err(_e) => cfg_file = or_file_name.to_owned(),
    }

    log_verbose!("get_toml", "file name: {}",cfg_file);

    let config_toml_content = fs::read_to_string(cfg_file).expect(error_msg.as_str());

    let t: toml::Value = toml::de::from_str(&config_toml_content).unwrap();
    t
}