use dirs;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;

type Outcome<T> = Result<T, ()>;
const INVALID_WP_NAME: &'static str = "is not a valid waypoint";

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
    pub fn rename(&self, name: &str) -> Self {
        Self {
            name: String::from(name),
            path: self.path.clone(),
            group: self.group.clone(),
        }
    }
    pub fn repath(&self, path: &str) -> Self {
        Self {
            name: self.name.clone(),
            path: String::from(path),
            group: self.group.clone(),
        }
    }
    pub fn regroup(&self, group: &str) -> Self {
        Self {
            name: self.name.clone(),
            path: self.path.clone(),
            group: Some(String::from(group)),
        }
    }
    pub fn ungroup(&self) -> Self {
        Self {
            name: self.name.clone(),
            path: self.path.clone(),
            group: None,
        }
    }
}

/// List
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct List(pub Vec<Waypoint>);

impl List {
    // query
    pub fn get_entry(&self, name: &str) -> Option<&Waypoint> {
        self.0.iter().find(|w| w.name == name)
    }

    pub fn get_group(&self, group: &str) -> Option<&Waypoint> {
        self.0.iter().find(|w| w.group == Some(group.to_string()))
    }

    fn get_index(&self, name: &str) -> Option<usize> {
        self.0.iter().position(|w| w.name == name)
    }

    // filter
    pub fn filter_group(&self, group: Option<&str>) -> Option<Self> {
        let g = group.map(str::to_string);
        let filtered_wps: Vec<Waypoint> = self.0.iter().filter(|w| w.group == g).cloned().collect();
        if !filtered_wps.is_empty() {
            Some(Self(filtered_wps))
        } else {
            None
        }
    }

    // delete
    pub fn remove_entries(mut self, names: Vec<&str>) -> Outcome<Self> {
        for n in names {
            let mut i = 0;
            while i != self.0.len() {
                if self.0.get(i).unwrap().name == n.to_string() {
                    let n = &self.0.get(i).unwrap().name.clone();
                    self.0.remove(i);
                    println!("'{}' removed from waypoints", &n)
                } else {
                    i += 1;
                }
            }
        }
        Ok(self)
    }

    pub fn remove_group(mut self, groups: Vec<&str>) -> Outcome<Self> {
        for g in groups {
            if self.get_group(g).is_some() {
                let mut i = 0;
                while i != self.0.len() {
                    if self.0.get(i).unwrap().group == Some(g.to_string()) {
                        let n = &self.0.get(i).unwrap().name.clone();
                        self.0.remove(i);
                        println!("'{}' removed from waypoints", &n)
                    } else {
                        i += 1;
                    }
                }
                println!("group '{}' removed", &g)

            } else {
                println!("group '{}' is not defined", &g)
            }
        }
        Ok(self)
    }

    // update
    pub fn rename_entry(mut self, name: &str, new_name: &str) -> Outcome<Self> {
        match self.get_index(name) {
            Some(i) => {
                let new_wp = self.0.get(i).unwrap().clone().rename(new_name);
                self.0.push(new_wp);
                self.0.remove(i);
                println!("'{}' renamed to '{}'", name, new_name);
                Ok(self)
            }
            None => Err(println!("'{}' {}", name, INVALID_WP_NAME)),
        }
    }

    pub fn repath_entry(mut self, name: &str, path: &str) -> Outcome<Self> {
        match self.get_index(name) {
            Some(i) => {
                let old_path = self.0.get(i).unwrap().path.clone();
                let new_wp = self.0.get(i).unwrap().clone().repath(path);
                self.0.remove(i);
                self.0.push(new_wp);
                println!(
                        "path changed for waypoint '{}':\n old: {}\n new: {}",
                        name, old_path, path
                    );
                Ok(self)
            }
            None => Err(println!("'{}' {}", name, INVALID_WP_NAME)),
        }
    }

    pub fn regroup_entry(mut self, name: &str, group: &str) -> Outcome<Self> {
        match self.get_index(name) {
            Some(i) => {
                if let Some(old_group) = self.0.get(i).unwrap().group.clone() {
                    let new_wp = self.0.get(i).unwrap().clone().regroup(group);
                    self.0.remove(i);
                    self.0.push(new_wp);
                    println!(
                        "'{}' has been regrouped from '{}' to '{}'",
                        name, old_group, group
                    );
                    Ok(self)
                } else {
                    let new_wp = self.0.get(i).unwrap().clone()
                        .regroup(group);
                    self.0.remove(i);
                    self.0.push(new_wp);
                    println!("'{}' has been added to group '{}'", name, group);
                    Ok(self)
                }
            }
            None => Err(println!("'{}' {}", name, INVALID_WP_NAME)),
        }
    }

    pub fn ungroup_entry(mut self, name: &str) -> Outcome<Self> {
        match self.get_index(name) {
            Some(i) => {
                if let Some(old_group) = self.0.get(i).unwrap().clone().group {
                    let new_wp = self.0.get(i).unwrap().clone()
                        .ungroup();
                    self.0.remove(i);
                    self.0.push(new_wp);
                    println!("'{}' has been removed from group '{}'", name, old_group);
                    Ok(self)
                } else {
                    Err(println!("'{}' does not have a group", name))
                }
            }
            None => Err(println!("'{}' {}", name, INVALID_WP_NAME)),
        }
    }

    /// Load `waypoints.json`
    pub fn load() -> List {
        // create file if it does not exist
        if fs::metadata(Self::path()).is_err() {
            fs::create_dir_all(Self::config_path())
                .expect("could note create directory '~/.config/tele'");
            fs::write(Self::path(), b"[]")
                .expect("could not create file '~/.config/tele/waypoints.json'")
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

    /// Load specific group
    pub fn load_group(group: &str) -> Option<List> {
        let list = Self::load();
        list.filter_group(Some(group))
    }

    /// Load waypoints without a group
    pub fn load_groupless() -> Option<List> {
        let list = Self::load();
        list.filter_group(None)
    }

    /// Sorts waypoints and writes List to `waypoints.json`
    pub fn save(mut self) {
        self.0.sort_by(|a, b| a.name.cmp(&b.name));
        self.0.sort_by(|a, b| a.group.cmp(&b.group));
        let json = serde_json::to_string_pretty(&self).expect("could not serialize input");
        fs::write(List::path(), json).expect("unable to write list");
    }

    /// PathBuf for `waypoints.json`
    /// ~/config/tele/
    fn config_path() -> PathBuf {
        dirs::home_dir()
            .expect("failed to access home directory")
            .join(".config")
            .join("tele")
    }
    /// ~/config/tele/waypoints.json
    fn path() -> PathBuf {
        dirs::home_dir()
            .expect("failed to access home directory")
            .join(Self::config_path())
            .join("waypoints.json")
    }
}
