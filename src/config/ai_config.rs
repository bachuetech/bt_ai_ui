use std::collections::HashMap;

use bt_logger::log_warning;
use yaml_rust2::Yaml;

use crate::utils::yaml_utils;
use crate::config::labels;

const AI_YML_CONFIG: &str = "config/ai-config.yml";
const AI_YML_CONFIG_ENV_VAR_NAME: &str = "AICONFIGYMLFILE";

const DEFAULT_NAME: &str = "JeremyBT";
const DEFAULT_PORT: i64 = 11434;

#[derive(Debug)]
struct _AIServer {
    host: String,
    port: u16,
    secure: bool,
}


#[derive(Debug)]

struct _AIApis {
    ctx_max: usize,
    path: String,
    chat: String,
    generate: String,
    models: String,
}

#[derive(Debug)]
pub struct AIConfig {
    //server: _AIServer,
    name: String,
    //api: _AIApis,
    //ai_url: String,
    platforms: HashMap<String, Platform>,
}


#[derive(Debug)]
struct Platform {
    //name: String,
    //server: _AIServer,
    api: _AIApis,
    ai_url: String,
    models: HashMap<String, Model>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SupportedFunctions {
    ALL,
    NONE,
    Functions(Vec<String>),
}

impl SupportedFunctions {
    /// Method to convert a comma-separated string into a list of names
    fn from_str_list(s: &str) -> Self {
        let names: Vec<String> = s.split(',')
            .map(|name| name.trim().to_string()) // Trim each name and convert to String
            .collect();
        SupportedFunctions::Functions(names)
    }
}

impl From<String> for SupportedFunctions {
    fn from(s: String) -> Self {
        match s.as_str() {
            "ALL" => SupportedFunctions::ALL,
            "NONE" => SupportedFunctions::NONE,
            _ => SupportedFunctions::from_str_list(&s), // Otherwise, treat it as a list of names
        }
    }
}

impl From<Yaml> for SupportedFunctions {
    fn from(s: Yaml) -> Self {
        match s.as_str() {
            Some("ALL") => SupportedFunctions::ALL,
            Some("NONE") => SupportedFunctions::NONE,
            _ => SupportedFunctions::Functions(yaml_utils::convert_yaml_to_vec_string(&s)), // Otherwise, treat it as a list of names
        }
    }
}

#[derive(Debug)]
pub struct Model{
    pub tool_support: bool,
    pub system: String,
    pub tools: SupportedFunctions,
}
pub enum InteractionType {
    Chat,
    Generate,
    Models,
}

impl AIConfig {
    // Constructor to read from YAML file
    pub fn new(run_env: String) -> Self {
        let ai_config = yaml_utils::get_yaml(
            AI_YML_CONFIG_ENV_VAR_NAME,
            AI_YML_CONFIG,
        );

        let mut platform_list: HashMap<String, Platform> = HashMap::new();
        for plat in ai_config[run_env.as_str()][labels::AI_PLATFORM_LABEL].clone() {

            let mut port = plat[labels::SERVER_LABEL][labels::PORT_LABEL].as_i64().unwrap_or(DEFAULT_PORT);
            port = if port < 0 || port > 65535 {
                DEFAULT_PORT
            } else {
                port
            };

            let host_data = _AIServer {
                host: plat[labels::SERVER_LABEL][labels::HOST_LABEL]
                    .as_str()
                    .unwrap_or("localhost")
                    .to_owned(),
                port: port as u16,
                secure: plat[labels::SERVER_LABEL]["secure"].as_bool().unwrap_or(true),
            };

            let api_data = _AIApis {
                ctx_max: usize::try_from(plat["api"]["ctx_max"].as_i64().unwrap_or(5))
                    .ok()
                    .expect("Maximun Size of Context (ctx_max) in AI YML config file is invalid"),
                path: plat["api"]["path"].as_str().unwrap_or("api").to_owned(),
                chat: plat["api"]["chat"].as_str().unwrap_or("chat").to_owned(),
                generate: plat["api"]["generate"].as_str().unwrap_or("generate").to_owned(),
                models: plat["api"]["models"].as_str().unwrap_or("models").to_owned(),
            };

            let mut url = format!("{}{}{}", host_data.host.clone(), ":", host_data.port);
            let end_point = format!("{}{}{}", "/", api_data.path.clone(), "/");

            if host_data.secure {
                url = format!("{}{}{}", "https://", url, end_point);
            } else {
                url = format!("{}{}{}", "http://", url, end_point);
            }

            let mut config_models: HashMap<String, Model> = HashMap::new();
            for m in plat["models"].clone() {
                config_models.insert(
                    m["model"].as_str().unwrap_or("default").to_owned(),
                    Model{
                        tool_support: m["tool_support"].as_bool().unwrap_or(false),
                        system: m["system"].as_str().unwrap_or("You are an AI assistance").to_owned(),
                        tools: SupportedFunctions::from(m["tools"].clone()),
                    },
                );
            }

            let p = Platform {
                api: api_data,
                ai_url: url,
                models: config_models,
            };

            platform_list.insert(
                plat["name"].as_str().unwrap_or("default").to_owned(),
                p,
            );

        }

        Self {
            name: ai_config["name"].as_str().unwrap_or(DEFAULT_NAME).to_owned(),
            platforms: platform_list,
        }
    }

    fn get_platform(&self, name: String) -> Option<&Platform> {
        self.platforms.get(&name)
    }

    pub fn get_name(&self) -> &String{
        &self.name
    }


    pub fn get_url(&self, platform_name: String, int_type: InteractionType) -> String {
        if let Some(p) = self.get_platform(platform_name) {
            return match int_type {
                //let api_url = match int_type {
                InteractionType::Chat => {
                    format!("{}{}", p.ai_url, p.api.chat.clone())
                }
                InteractionType::Generate => {
                    format!("{}{}", p.ai_url.clone(), p.api.generate.clone())
                }
                InteractionType::Models => {
                    format!("{}{}", p.ai_url.clone(), p.api.models.clone())
                }
            };
        } else {
            log_warning!("get_url","Platform NOT found. Using default values!");
            return match int_type { //Default Values!
                InteractionType::Chat => "http://localhost/chat".to_owned(),
                InteractionType::Generate => "http://localhost/generate".to_owned(),
                InteractionType::Models => "http://localhost/models".to_owned(),
            };
        }


    }

    pub fn get_platform_list(&self) -> Vec<String>{
        self.platforms.keys().cloned().collect()
    }

    pub fn get_models(&self, platform_name: String) ->  Option<&HashMap<String, Model>>{
        if let Some(p) = self.get_platform(platform_name) {
            return Some(&p.models);
        }

        None
    }

    pub fn get_max_ctx_size(&self, platform_name: String) -> usize {
        if let Some(p) = self.get_platform(platform_name) {
            p.api.ctx_max as usize
        }else{
            1
        }
    }

}