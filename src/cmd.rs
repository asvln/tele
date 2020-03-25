use crate::cli::{EditMatches, ListMatches};
use crate::table;
use crate::waypoints::{List, Waypoint};
use crate::config::Filesystem;

pub fn add(name: &str, group: Option<&str>) {
    let mut list = List::load();
    match list.get_entry(&name) {
        Some(w) => println!("'{}' is already assigned to: {}", &name, &w.path),
        None => {
            let w = Waypoint::new(&name, group);
            list.0.push(w);
            list.save();
            println!("'{}' added to waypoints", &name)
        }
    }
}

pub fn rm(names: Option<Vec<&str>>, groups: Option<Vec<&str>>) {
    if let Some(req) = names {
        let list = List::load();
        if let Ok(l) = list.remove_entries(req) {
            l.save()
        }
    }
    if let Some(req) = groups {
        let list = List::load();
        if let Ok(l) = list.remove_group(req) {
            l.save()
        }
    }
}

pub fn dissolve(groups: Vec<&str>) {
    let list = List::load();
    if let Ok(l) = list.dissolve_groups(groups) {
        l.save()
    }
}

pub fn edit(wp: &str, kind: EditMatches) {
    match kind {
        EditMatches::Name(name) => {
            let list = List::load();
            if let Ok(l) = list.rename_entry(wp, &name) {
                l.save()
            }
        }
        EditMatches::Path(path) => {
            let list = List::load();
            if let Some(p) = path {
                if let Ok(l) = list.repath_entry(wp, &p) {
                    l.save()
                }
            } else {
                if let Ok(l) = list.repath_entry(wp, &Filesystem::current_dir()) {
                    l.save()
                }
            }
        }
        EditMatches::Group(group) => {
            let list = List::load();
            if let Ok(l) = list.regroup_entry(wp, &group) {
                l.save()
            }
        }
        EditMatches::Ungroup => {
            let list = List::load();
            if let Ok(l) = list.ungroup_entry(wp) {
                l.save()
            }
        }
    }
}

pub fn reload_list() {
    let list = List::load();
    list.save()
}

pub fn list(kind: ListMatches) {
    match kind {
        ListMatches::All => {
            let list = List::load();
            if list.0.is_empty() {
                println!("no waypoints defined")
            } else {
                table::print_all(list)
            }
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
                self::list(ListMatches::All)
            }
        }
    }
}

pub fn tele(name: &str) {
    let list = List::load();
    match list.get_entry(name) {
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
