use lazy_static::lazy_static;
use serde::Deserialize;
use std::{self, collections::HashMap, error::Error, fs, path::PathBuf};

use crate::constants::{CONFIG_NAME, CONFIG_PATH_ENV_KEY};

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub file_types: HashMap<String, Vec<String>>,
    pub db_override_path: Option<String>,
}

impl Config {
    fn update(&mut self, other: &Config) -> &mut Self {
        self.file_types = other.file_types.clone();
        self.db_override_path = other.db_override_path.clone();
        self
    }

    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut config_paths: Vec<PathBuf> = Vec::new();

        let default_config_path = PathBuf::from(CONFIG_NAME);
        if default_config_path.exists() {
            config_paths.push(default_config_path);
        }

        if let Ok(env_config_paths) = std::env::var(CONFIG_PATH_ENV_KEY) {
            for mut path in std::env::split_paths(&env_config_paths) {
                path.push(CONFIG_NAME);
                if path.exists() {
                    config_paths.push(path)
                }
            }
        }

        let mut result_config = Config::default();
        for config_path in config_paths {
            let config: Config = toml::from_str(&fs::read_to_string(config_path)?)?;
            result_config.update(&config);
        }

        Ok(result_config)
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new().expect("Failed to get global config");
}
