use serde::Deserialize;
use std::{error::Error, fs, path::PathBuf};

use crate::constants::{CONFIG_NAME, CONFIG_PATH_ENV_KEY};

// NOTE may be only server side
#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub storage_dir: String,
}

impl Config {
    fn update(&mut self, other: &Config) -> &mut Self {
        self.storage_dir = other.storage_dir.clone();
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
