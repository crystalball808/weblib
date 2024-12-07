use core::fmt;
use dirs;
use std::{fs, io, path::PathBuf};

pub const APP_NAME: &str = "Weblib";
const CONFIG_FILE_NAME: &str = "config.toml";

fn get_config_path() -> Option<PathBuf> {
    let mut config_path = dirs::config_dir()?;
    config_path.push(APP_NAME);
    config_path.push(CONFIG_FILE_NAME);

    Some(config_path)
}

pub struct Config {
    vault_path: Option<PathBuf>,
}
impl Default for Config {
    fn default() -> Self {
        Self { vault_path: None }
    }
}

pub fn get_config() -> Result<Config, Error> {
    let config_path = get_config_path().ok_or(Error::GetConfigPathFailed)?;

    if fs::exists(config_path)? {
        let vault_path = Some(PathBuf::from("foo"));
        Ok(Config { vault_path })
    } else {
        Ok(Config::default())
    }
}

// ==== Error ====
#[derive(Debug)]
pub enum Error {
    GetConfigPathFailed,
    IoError(io::Error),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::GetConfigPathFailed => write!(f, "Failed to get the config path"),
            Error::IoError(io_error) => write!(f, "Io Error: {}", io_error),
        }
    }
}
impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Error::IoError(io_error)
    }
}
