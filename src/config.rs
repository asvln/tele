use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use std::env;
use std::path::PathBuf;
use dirs;
/// Filesystem
pub struct Filesystem();

impl Filesystem {
    /// Returns the current working directory as a String
    pub fn current_dir() -> String {
        env::current_dir()
            .expect("invalid working directory")
            .into_os_string()
            .into_string()
            .expect("cannot parse working directory")
    }
    /// Returns the current working folder name as a String
    pub fn current_dir_name() -> String {
        let c = env::current_dir().expect("invalid working directory");
        c.file_name()
            .expect("invalid directory name")
            .to_str()
            .expect("cannot parse working directory")
            .to_string()
    }

    /// PathBuf for config folder
    /// ~/.config/tele/
    pub fn config_path() -> PathBuf {
        dirs::home_dir()
            .expect("failed to access home directory")
            .join(".config")
            .join("tele")
    }

    /// PathBuf for `config.json`
    /// ~/.config/tele/config.json
    pub fn config_file() -> PathBuf {
        dirs::home_dir()
            .expect("failed to access home directory")
            .join(Self::config_path())
            .join("config.json")
    }

    /// PathBuf for `config.json`
    /// ~/.config/tele/config.json
    pub fn waypoints_file() -> PathBuf {
        dirs::home_dir()
            .expect("failed to access home directory")
            .join(Filesystem::config_path())
            .join("waypoints.json")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config(HashMap<String, String>);

impl Config {
    /// Load `config.json`
    pub fn load() -> Config {
        // create file if it does not exist
        if fs::metadata(Filesystem::config_file()).is_err() {
            fs::create_dir_all(Filesystem::config_path())
                .expect("could note create directory '~/.config/tele'");
            fs::write(Filesystem::config_file(), b"{}")
                .expect("could not create file '~/.config/tele/config.json'")
        }
        // read file
        let mut file_string = String::new();
        File::open(Filesystem::config_file())
            .expect("error opening waypoint list")
            .read_to_string(&mut file_string)
            .expect("error converting list to string");
        // deserialize
        serde_json::from_str(&file_string).expect("error deserializing list")
    }

    pub fn save(self) {
        let json = serde_json::to_string_pretty(&self).expect("could not serialize input");
        fs::write(Filesystem::config_file(), json).expect("unable to write config");
    }

    pub fn check(key: &str) -> Option<String> {
        let config = Self::load();
        if let Some(v) = config.0.get(key) {
            Some(v.clone().to_owned())
        } else {
            None
        }
    }

    pub fn set(key: String, value: String) {
        let mut config = Self::load();
        config.0.insert(key, value);
        Self::save(config)
    }
}
