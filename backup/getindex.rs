/// Get index of Waypoint by `name`
pub fn get_index(&self, name: &str) -> Option<usize> {
    self.0.iter().position(|w| w.name == name)
}
