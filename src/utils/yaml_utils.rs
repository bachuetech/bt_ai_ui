    use std::{env, fs};
    use bt_logger::log_verbose;
    use yaml_rust2::{Yaml, YamlLoader};
    
    pub fn get_yaml(env_variable: &str, or_file_name: &str) -> Yaml{
        let cfg_file: String;
        let error_msg = format!("Unable to read config YML file from env variable: {} or default file: {}.", &env_variable, &or_file_name );

        match env::var(env_variable) {
            Ok(value) => cfg_file = value,
            Err(_e) => cfg_file = or_file_name.to_owned(),
        }

        log_verbose!("get_yaml", "file name: {}",cfg_file);

        let config_yml_content =
            fs::read_to_string(cfg_file).expect(error_msg.as_str()); //"Unable to read config YML file from env{} or file {}.", &env_variable, &or_file_name );
        let yml_config = YamlLoader::load_from_str(&config_yml_content).unwrap();
        yml_config[0].clone()
    }