use super::{constants::APPLICATION_CONFIGURATION_FILE, LibraryError};
use serde::Deserialize;
use std::{env::current_dir, fs::read_to_string, path::Path};

#[derive(Debug, Deserialize)]
pub struct Config {
    commands: Option<CommandsConfig>,
}

impl Config {
    pub(crate) fn from_current_directory() -> Result<Self, LibraryError> {
        let config_file = read_to_string(
            Path::new(&current_dir().unwrap()).join(APPLICATION_CONFIGURATION_FILE),
        );
        match config_file {
            Ok(data) => {
                let config = toml::from_str(&data).unwrap();
                Ok(config)
            }
            Err(_) => {
                trace!(
                    "Unable to find file: {:?}, using default configuration",
                    APPLICATION_CONFIGURATION_FILE
                );
                Err(LibraryError)
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct CommandsConfig {
    package: Option<PackageCommandsConfig>,
    graph: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PackageCommandsConfig {
    manager: Option<String>,
}
