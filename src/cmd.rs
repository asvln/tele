use crate::cli::{EditMatches, ListMatches};
use crate::table;
use crate::waypoints::{Filesystem, List, Waypoint};

pub fn add(name: &str, group: Option<&str>) {
    let list = List::load();
    match list.get_waypoint(&name) {
        Some(w) => println!("'{}' is already assigned to: {}", &name, &w.path),
        None => {
            let w = Waypoint::new(&name, group);
            let l = list.append_entry(w).clone();
            List::save(l);
            println!("'{}' added to waypoints", &name)
        }
    }
}

pub fn rm(name: Option<&str>, group: Option<&str>) {
    if let Some(n) = name {
        let list = List::load();
        match list.remove_entry(&n) {
            Ok(l) => List::save(l),
            Err(e) => println!("{}", e),
        }
    }
    if let Some(g) = group {
        let list = List::load();
        match list.remove_group(&g) {
            Ok(l) => List::save(l),
            Err(e) => println!("{}", e),
        }
    }
}

pub fn edit(wp: &str, kind: EditMatches) {
    match kind {
        EditMatches::Name(name) => {
            let list = List::load();
            match list.rename_entry(wp, &name) {
                Ok(l) => List::save(l),
                Err(e) => println!("{}", e),
            }
        }
        EditMatches::Path(path) => {
            let list = List::load();
            if let Some(p) = path {
                match list.repath_entry(wp, &p) {
                    Ok(l) => List::save(l),
                    Err(e) => println!("{}", e),
                }
            } else {
                match list.repath_entry(wp, &Filesystem::current_dir()) {
                    Ok(l) => List::save(l),
                    Err(e) => println!("{}", e),
                }
            }
        }
        EditMatches::Group(group) => {
            let list = List::load();
            match list.regroup_entry(wp, &group) {
                Ok(l) => List::save(l),
                Err(e) => println!("{}", e),
            }
        }
        EditMatches::Ungroup => {
            let list = List::load();
            match list.ungroup_entry(wp) {
                Ok(l) => List::save(l),
                Err(e) => println!("{}", e),
            }
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
            if let Some(l) = list {
                table::print_group(l, &g)
            } else {
                println!("'{}' is an empty group", &g)
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
        None => println!("'{}' is not assigned", name),
    }
}

pub fn parse_name(name: Option<&str>) -> String {
    name.unwrap_or(&Filesystem::current_dir_name()).to_string()
}
