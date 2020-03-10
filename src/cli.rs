use crate::cmd;
use clap::{App, AppSettings, Arg, SubCommand};

pub fn parse_args() -> clap::ArgMatches<'static> {
    App::new("tele")
        .version(crate_version!())
        .about("Quickly `cd` into commonly used directories.")
        .settings(&[
            AppSettings::GlobalVersion,
            AppSettings::ArgRequiredElseHelp,
            AppSettings::ColorAlways,
            AppSettings::ColoredHelp,
            AppSettings::SubcommandsNegateReqs,
            AppSettings::VersionlessSubcommands,
            AppSettings::UnifiedHelpMessage,
            AppSettings::DeriveDisplayOrder,
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
        // add
        .subcommand(
            SubCommand::with_name("add")
                .about("Add working directory to waypoints")
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
                    // .multiple(true)
                ),
        )
        // rm
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove from waypoints")
                .arg(
                    Arg::with_name("name")
                        .help("Waypoint name to be removed")
                        .index(1)
                        .required_unless("group")
                        .conflicts_with("group"),
                )
                .arg(
                    Arg::with_name("group")
                        .help("Remove all specified group entries")
                        .short("g")
                        .long("group")
                        .takes_value(true)
                        .empty_values(false),
                    // .multiple(true),
                ),
        )
        // edit
        .subcommand(
            SubCommand::with_name("edit")
                .about("Edit an existing waypoint or group")
                .arg(
                    Arg::with_name("name")
                        .help("Define a custom waypoint name (defaults to current folder name)")
                        .short("n")
                        .long("name")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("group")
                        .help("Add waypoint to a custom group")
                        .short("g")
                        .long("group")
                        .takes_value(true),
                    // .multiple(true)
                ),
        )
        // list
        .subcommand(
            SubCommand::with_name("list")
                .about("Print waypoints")
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
            let name = matches.value_of("name");
            let group = matches.value_of("group");
            cmd::rm(name, group)
        }
        ("list", Some(matches)) => {
            if matches.is_present("all") {
                cmd::list(ListMatches::All)
            } else if matches.is_present("group") {
                let group = matches.value_of("group");
                cmd::list(ListMatches::Group(group.unwrap().to_string()))
            } else {
                cmd::list(ListMatches::Groupless)
            }
        }
        ("", None) => {
            let name = matches.value_of("WAYPOINT").unwrap();
            cmd::tele(name)
        }
        _ => unreachable!(),
    }
}
