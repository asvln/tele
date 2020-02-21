use crate::table;
use crate::waypoints::{List, Waypoint, Filesystem};

pub fn add(name: &str, group: Option<&str>) {
    let list = List::load();

    match list.get_waypoint(&name) {
        Some(w) => println!("'{}' is already assigned to {}", &name, &w.path),
        None => {
            let w = Waypoint::new(&name, group);
            let out = list.append_entry(w).clone();
            List::save(&out)
        }
    }
}

pub fn rm(name: Option<&str>, group: Option<&str>) {
    if name.is_some() {
        let list = List::load();
        match list.remove_entry(&name.unwrap()) {
            Ok(l) => List::save(&l),
            Err(e) => println!("{}", e)
        }
    }
    if group.is_some() {
        let list = List::load();
        match list.remove_group(&name.unwrap()) {
            Ok(l) => List::save(&l),
            Err(e) => println!("{}", e)
        }
    }
}

pub fn list(group: Option<&str>) {
    let list = List::load();
        match list.filter_group(group) {
            Some(w) => table::print(w, group),
            None => println!("no entries for group '{}'", group.unwrap_or("undefined"))
        }
}


pub fn list_all() {
    let list = List::load();
    table::print(list, None)
    // table::print_all(list.0)
}

pub fn tele(name: &str) {
    let list = List::load();
    match list.get_waypoint(name) {
        Some(w) => {
            println!("{}", &w.path);
            std::process::exit(2)
        }
        None => println!("'{}' is not an assigned group", name)
    }
}

pub fn parse_name(name: Option<&str>) -> String {
    name.unwrap_or(&Filesystem::current_dir_name()).to_string()
}
