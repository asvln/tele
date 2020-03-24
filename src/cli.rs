use crate::cmd;
use clap::{App, AppSettings, Arg, SubCommand};

macro_rules! global_settings {
    () => {
        &[
            AppSettings::UnifiedHelpMessage,
            AppSettings::DeriveDisplayOrder,
            AppSettings::ColoredHelp,
            AppSettings::ColorAlways,
        ]
    }
}
pub fn parse_args() -> clap::ArgMatches<'static> {
    App::new("tele")
        .version(crate_version!())
        .about("Quickly `cd` into commonly used directories.")
        .settings(global_settings!())
        .settings(&[
            AppSettings::GlobalVersion,
            AppSettings::ArgRequiredElseHelp,
            AppSettings::SubcommandsNegateReqs,
            AppSettings::VersionlessSubcommands,
        ])
        // tele
        .arg(
            Arg::with_name("WAYPOINT")
                .help("Waypoint to tele to...")
                .index(1)
                .required_unless("add")
                .required_unless("rm")
                .required_unless("list"),
        )
        // edit
        .arg(
            Arg::with_name("name")
                .help("Change waypoint name")
                .short("n")
                .long("name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("path")
                .help("Change waypoint path (default is working directory)")
                .short("p")
                .long("path"),
        )
        .arg(
            Arg::with_name("group")
                .help("Change waypoint group")
                .short("g")
                .long("group")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ungroup")
                .help("Remove waypoint from it's group")
                .short("u")
                .long("ungroup"),
        )
        // add
        .subcommand(
            SubCommand::with_name("add")
                .about("Add working directory to waypoints")
                .settings(global_settings!())
                .arg(
                    Arg::with_name("name")
                        .help("Name of waypoint")
                        .index(1),
                )
                .arg(
                    Arg::with_name("group")
                        .help("Add waypoint to a custom group")
                        .short("g")
                        .long("group")
                        .takes_value(true),
                ),
        )
        // rm
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove waypoints")
                .settings(global_settings!())
                .arg(
                    Arg::with_name("name")
                        .help("Waypoint names to be removed")
                        .multiple(true)
                        .required_unless("group")
                        .required_unless("dissolve"),
                )
                .arg(
                    Arg::with_name("group")
                        .help("Remove waypoints in specified groups")
                        .short("g")
                        .long("group")
                        .multiple(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("dissolve")
                        .help("Dissolve a group (retains waypoints)")
                        .short("d")
                        .long("dissolve")
                        .multiple(true)
                        .takes_value(true),
                ),
        )
        // list
        .subcommand(
            SubCommand::with_name("list")
                .about("Print waypoints")
                .settings(global_settings!())
                .arg(
                    Arg::with_name("group")
                        .help("List only specified group")
                        .short("g")
                        .long("group")
                        .takes_value(true)
                        .empty_values(false),
                )
                .arg(
                    Arg::with_name("all")
                        .help("List all waypoints")
                        .short("a")
                        .long("all"),
                ),
        )
        .get_matches()
}

pub enum EditMatches {
    Name(String),
    Path(Option<String>),
    Group(String),
    Ungroup,
}

pub enum ListMatches {
    Groupless,
    All,
    Group(String),
}

pub fn parse_matches(matches: clap::ArgMatches<'static>) {
    match matches.subcommand() {
        ("add", Some(matches)) => {
            let name = cmd::parse_name(matches.value_of("name"));
            let group = matches.value_of("group");
            cmd::add(&name, group)
        }
        ("rm", Some(matches)) => {
            if matches.is_present("name") {
                let names: Vec<_> = matches.values_of("name").unwrap().collect();
                cmd::rm(Some(names), None)
            }
            if matches.is_present("group") {
                let groups: Vec<_> = matches.values_of("group").unwrap().collect();
                cmd::rm(None, Some(groups))
            }
            if matches.is_present("dissolve") {
                let groups: Vec<_> = matches.values_of("dissolve").unwrap().collect();
                cmd::dissolve(groups)
            }
        }
        ("list", Some(matches)) => {
            if matches.is_present("all") {
                cmd::list(ListMatches::All)
            } else if matches.is_present("group") {
                let group = matches.value_of("group").unwrap();
                cmd::list(ListMatches::Group(group.to_string()))
            } else {
                cmd::list(ListMatches::Groupless)
            }
        }
        ("", None) => {
            let wp = matches.value_of("WAYPOINT").unwrap();
            if matches.is_present("name") {
                let name = matches.value_of("name").unwrap();
                cmd::edit(wp, EditMatches::Name(name.to_string()))
            } else if matches.is_present("path") {
                let path = matches.value_of("path").map(str::to_string);
                cmd::edit(wp, EditMatches::Path(path))
            } else if matches.is_present("group") {
                let group = matches.value_of("group").unwrap();
                cmd::edit(wp, EditMatches::Group(group.to_string()))
            } else if matches.is_present("ungroup"){
                cmd::edit(wp, EditMatches::Ungroup)
            } else {
                cmd::tele(wp)
            }
        }
        _ => unreachable!()
    }
}
