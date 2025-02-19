use std::process;

use bt_logger::log_fatal;
use bt_yaml_utils::get_yaml;
use yaml_rust2::Yaml;

use crate::process_exit_codes::SERVER_CONFIG_READING_ERROR;

const SRV_YML_CONFIG: &str = "config/server-config.yml";
const SRV_YML_CONFIG_ENV_VAR_NAME: &str = "SRVCONFIGYMLFILE";

pub struct ServerConfig {
    host: String,
    port: u16,
    secure: bool,
}

impl ServerConfig {
    // Constructor to read from YAML file
    pub(super) fn new(run_env: String) -> Self {
        let srv_config: Yaml;
        match get_yaml(SRV_YML_CONFIG_ENV_VAR_NAME, SRV_YML_CONFIG){
            Ok(y_file_conf) => srv_config = y_file_conf,
            Err(e) => {log_fatal!("new","Fatal Error Reading SERVER configuration. Application aborted! {}",e.to_string()); process::exit(SERVER_CONFIG_READING_ERROR);}, // Exit the program with code -103
        }

        let mut srv_port = srv_config[run_env.as_str()]["server"]["port"]
            .as_i64()
            .unwrap();
        srv_port = if srv_port < 0 || srv_port > 65535 {
            3000
        } else {
            srv_port
        };

        Self {
            host: srv_config[run_env.as_str()]["server"]["host"]
                .as_str()
                .unwrap()
                .to_string(),
            port: srv_port as u16,
            secure: srv_config[run_env.as_str()]["server"]["secure"]
                .as_bool()
                .unwrap_or(true),
        }
    }

    pub fn get_tcp_listener(&self) -> String {
        format!("{}:{}", self.host.clone(), self.port)
    }

    pub fn is_secure(&self) -> bool {
        self.secure
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
}

pub fn get_srv_config(current_env: String) -> ServerConfig {
    ServerConfig::new(current_env)
}
