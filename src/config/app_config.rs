use std::collections::HashMap;

use crate::utils::yaml_utils;

const APP_YML_CONFIG: &str = "config/app-config.yml";
const APP_YML_CONFIG_ENV_VAR_NAME: &str = "APPCONFIGYMLFILE";

const APP_DEFAULT_NAME: &str = "BACHUETECH";
const APP_DEFAULT_VERSION: &str = "x0.0.1d";

//const CARGO_TOML_FILE: &str = "Cargo.toml";

#[derive(Clone)]
pub struct AppConfig {
    name: String,
    version: String,
    environment: String,
    files_app_dir: String,
    app_path: String,
    api_path: String,
    end_points: HashMap<String, String>
}

impl AppConfig {
    // Constructor to read from YAML file
    pub fn new() -> Self {
        let app_config = yaml_utils::get_yaml(APP_YML_CONFIG_ENV_VAR_NAME, APP_YML_CONFIG);

        let app_environment = app_config["environment"].as_str().unwrap_or("DEV");

        let mut end_points = HashMap::new();
        for value in app_config[app_environment]["end_points"].clone() {
            end_points.insert(
                value["id"].as_str().unwrap().to_string(),
                value["path"].as_str().unwrap().to_string(),
            );
        }

        //let cargo_toml = toml_utils::get_toml("", CARGO_TOML_FILE);
        //let mut package_name: String = "Unknown".to_owned();
        //let mut package_version: String = "x0.0.1d".to_owned();

        // Extract package name and version from Cargo file
        /*if let Some(package) = cargo_toml.get("package") {
            if let Some(name) = package.get("name") {
                if let Some(version) = package.get("version") {
                    package_name = name.as_str().unwrap_or("Unknown").to_string();
                    package_version = version.as_str().unwrap_or("x0.0.1d").to_string();
                }
            }
        }*/

        //Allows to overwrite Cargo file info with App Confi Yaml info if necessary
        let app_name = app_config["app_name"]
            .as_str()
            .unwrap_or(APP_DEFAULT_NAME);
        let app_ver = app_config["version"]
            .as_str()
            .unwrap_or(APP_DEFAULT_VERSION);

        Self {
            name: app_name.to_owned(),
            version: app_ver.to_owned(),
            environment: app_environment.to_owned(),
            files_app_dir: app_config[app_environment]["files_app_dir"]
                .as_str()
                .unwrap()
                .to_string(),
            app_path: app_config[app_environment]["app_path"]
                .as_str()
                .unwrap()
                .to_string(),
            api_path: app_config[app_environment]["api_path"]
                .as_str()
                .unwrap()
                .to_string(),
            end_points: end_points,
        }
    }

    pub fn get_environment(&self) -> String {
        self.environment.clone()
    }

    pub fn get_file_app_dir(&self) -> String {
        self.files_app_dir.clone()
    }

    pub fn get_app_path(&self) -> String {
        self.app_path.clone()
    }

    pub fn get_api_path(&self) -> String {
        self.api_path.clone()
    }

    pub fn get_end_point(&self, end_point_name: &str) -> String {
        self.end_points
            .get(end_point_name)
            .unwrap_or(&"/".to_string())
            .to_string()
    }

    pub fn get_app_name(&self) -> &String {
        &self.name
    }

    pub fn get_version(&self) -> &String {
        &self.version
    }
}
