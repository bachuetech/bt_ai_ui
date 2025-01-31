use std::{env, fs};

use bt_logger::log_trace;

pub fn get_file(env_variable: &str, or_file_name: &str) -> String{
    //let log = get_logger();
    let cfg_file: String;
    let error_msg = format!("Unable to read JSON file from env variable: {} or default file: {}.", &env_variable, &or_file_name );

    match env::var(env_variable) {
        Ok(value) => cfg_file = value,
        Err(_e) => cfg_file = or_file_name.to_owned(),
    }

    //log.trace("get_json", &format!("file name: {}",cfg_file));
    log_trace!("get_file", "file name: {}",cfg_file);

    fs::read_to_string(cfg_file).expect(error_msg.as_str()) //"Unable to read config YML file from env{} or file {}.", &env_variable, &or_file_name );
}