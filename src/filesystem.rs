use std::env;
use std::path::PathBuf;
use crate::TELE;

/// Filesystem helpers
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

    /// PathBuf for `waypoints.json`
    /// ~/.config/tele/waypoints.json
    pub fn waypoints_file() -> PathBuf {
        TELE.config_dir().join("waypoints.json")
    }
}
