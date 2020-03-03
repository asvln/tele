use dirs;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;

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
    pub fn current_dir_name() -> String {
        let c = env::current_dir().expect("invalid working directory");
        c.file_name()
            .expect("invalid directory name")
            .to_str()
            .expect("cannot parse working directory")
            .to_string()
    }
}

/// Waypoint
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Waypoint {
    pub name: String,
    pub path: String,
    pub group: Option<String>,
}

impl Waypoint {
    pub fn new(name: &str, group: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            path: Filesystem::current_dir(),
            group: group.map(str::to_string),
        }
    }
}

/// List
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct List(pub Vec<Waypoint>);

impl List {
    pub fn get_waypoint(&self, name: &str) -> Option<&Waypoint> {
        self.0.iter().find(|w| w.name == name)
    }

    pub fn get_index(&self, name: &str) -> Option<usize> {
        self.0.iter().position(|w| w.name == name)
    }

    pub fn filter_group(&self, group: Option<&str>) -> Option<Self> {
        let g = group.map(str::to_string);
        let filtered_wps: Vec<Waypoint> = self.0.iter().filter(|w| w.group == g).cloned().collect();
        if !filtered_wps.is_empty() {
            Some(Self(filtered_wps))
        } else {
            None
        }
    }

    pub fn append_entry(self, wp: Waypoint) -> Self {
        let mut wps = self.0;
        wps.push(wp);
        Self(wps)
    }

    pub fn remove_entry(mut self, name: &str) -> Result<Self, &str> {
        match self.get_index(name) {
            Some(i) => {
                self.0.remove(i);
                println!("'{}' removed from waypoints", name);
                Ok(self)
            }
            None => Err("waypoint was not found"),
        }
    }

    pub fn remove_group(mut self, name: &str) -> Result<Self, &str> {
        match self.get_index(name) {
            Some(i) => {
                self.0.remove(i);
                println!("'{}' removed from waypoints", name);
                Ok(self)
            }
            None => Err("no group entries found"),
        }
    }

    /// File operations:

    /// Reads `waypoints.json`
    pub fn load() -> List {
        // create file if it does not exist
        if fs::metadata(Self::path()).is_err() {
            fs::write(Self::path(), b"[]")
                .expect("could not create '~/.config/tele/waypoints.json'")
        }
        // read file
        let mut file_string = String::new();
        File::open(Self::path())
            .expect("error opening waypoint list")
            .read_to_string(&mut file_string)
            .expect("error converting list to string");
        // deserialize
        serde_json::from_str(&file_string).expect("error deserializing list")
    }

    pub fn load_group(group: &str) -> Option<List> {
        let list = Self::load();
        list.filter_group(Some(group))
    }

    pub fn load_groupless() -> Option<List> {
        let list = Self::load();
        list.filter_group(None)
    }

    /// Writes `waypoints.json`
    pub fn save(wps: &List) {
        // serialize
        let json = serde_json::to_string_pretty(wps).expect("could not serialize input");
        // write file
        fs::write(List::path(), json).expect("unable to write list");
    }

    /// PathBuf for `waypoints.json`
    fn path() -> PathBuf {
        // ~/config/tele/waypoints.json
        dirs::home_dir()
            .expect("failed to access home directory")
            .join(".config")
            .join("tele")
            .join("waypoints.json")
    }
}
