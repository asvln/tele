use crate::table;
use crate::waypoints::{List, Waypoint, Filesystem};
use crate::cli::ListMatches;

pub fn add(name: &str, group: Option<&str>) {
    let list = List::load();

    match list.get_waypoint(&name) {
        Some(w) => println!("'{}' is already assigned to: {}", &name, &w.path),
        None => {
            let w = Waypoint::new(&name, group);
            let out = list.append_entry(w).clone();
            List::save(&out);
            println!("waypoint added")
        }
    }
}

pub fn rm(name: Option<&str>, group: Option<&str>) {
    if let Some(n) = name {
        let list = List::load();
        match list.remove_entry(&n) {
            Ok(l) => List::save(&l),
            Err(e) => println!("{}", e)
        }
    }
    if let Some(g) = group {
        let list = List::load();
        match list.remove_group(&g) {
            Ok(l) => List::save(&l),
            Err(e) => println!("{}", e)
        }
    }
}

pub fn list(kind: ListMatches) {
    match kind {
        ListMatches::All => {
            let list = List::load();
            table::print_all(list)
        }
        ListMatches::Group(g) => {
            let list = List::load_group(&g);
            if list.0.is_empty() {
                println!("no entries for group '{}'", &g)
            } else {
                table::print_group(list, &g)
            }
        }
        ListMatches::Groupless => {
            let list = List::load_groupless();
            if let Some(l) = list {
                table::print_groupless(l)
            } else {
                println!("no waypoints defined")
            }
        }
    }
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
