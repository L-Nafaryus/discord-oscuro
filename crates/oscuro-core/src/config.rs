use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub discord_token: String,
}

impl Config {
    pub fn data_dir() -> Result<std::path::PathBuf, ConfigError> {
        let cwd = std::env::current_dir()?;
        if cfg!(debug_assertions) {
            Ok(cwd.join("temp"))
        } else {
            Ok(cwd)
        }
    }

    pub fn open(path: &std::path::Path) -> Result<Config, ConfigError> {
        fs::read_to_string(path)?.parse()
    }

    pub fn to_string(&self) -> Result<String, ConfigError> {
        Ok(toml::to_string(self)?)
    }

    pub fn write(&self, path: &std::path::Path) -> Result<(), ConfigError> {
        Ok(fs::write(path, self.to_string()?)?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            discord_token: String::from("Bot ###"),
        }
    }
}

impl std::str::FromStr for Config {
    type Err = ConfigError;
    fn from_str(s: &str) -> Result<Self, ConfigError> {
        toml::from_str(s).map_err(|_| ConfigError::Parse)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Parse,
    StringParse,
    Serialize,
    IO,
}

impl std::error::Error for ConfigError {}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Parse => write!(f, "Failed to parse Config from string"),
            Self::StringParse => write!(f, "Failed to parse environment variable"),
            Self::Serialize => write!(f, "Failed to serialize Config to TOML"),
            Self::IO => write!(f, "Faild to write file"),
        }
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(_: toml::ser::Error) -> Self {
        ConfigError::Serialize
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(_: std::io::Error) -> Self {
        ConfigError::IO
    }
}

impl From<std::num::ParseIntError> for ConfigError {
    fn from(_: std::num::ParseIntError) -> Self {
        ConfigError::StringParse
    }
}
