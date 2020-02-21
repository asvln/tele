
#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    path: PathBuf,
    waypoints: Vec<Waypoint>,
}

impl List {
    // Default path
    fn path() -> PathBuf {
        dirs::home_dir()
            .expect("failed to access home directory")
            .join(".config")
            .join("tele")
            .join("waypoints.json")
    }
    // Get and deserialize waypoints to Vec<Waypoint>
    pub fn waypoints() -> Vec<Waypoint> {
        init();
        let mut wps = String::new();
        File::open(List::path())
            .expect("error opening waypoint list")
            .read_to_string(&mut wps)
            .expect("error converting list to string");
        serde_json::from_str(&wps)
            .expect("error deserializing list")
    }

}

impl Default for List {
    fn default() -> List {
        List {
            path: List::path(),
            waypoints: List::waypoints(),
        }
    }
}
