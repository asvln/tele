use clap::{App, AppSettings, Arg, SubCommand};
use crate::cmd;

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
                .required_unless("list")
        )
        // add
        .subcommand(
            SubCommand::with_name("add")
                .about("Add working directory to waypoints")
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
                )
        )
        // rm
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove from waypoints")
                .arg(
                    Arg::with_name("name")
                    .help("Waypoint name to be removed")
                    .index(1)
                    .required(true),
                )
                .arg(
                    Arg::with_name("group")
                        .help("Remove all specified group entries")
                        .short("g")
                        .long("group")
                        .takes_value(true),
                        // .multiple(true),
                )
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
                    .takes_value(true),
                )
                .arg(
                    Arg::with_name("all")
                    .help("List all waypoints")
                    .short("a")
                    .long("all"),
                )
        )
        .get_matches()
}

pub fn parse_matches(matches: clap::ArgMatches<'static>) {
    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            let name = add_matches.value_of("name");
            let group = add_matches.value_of("group");
            cmd::add(&cmd::parse_name(name), group)
        }
        ("rm", Some(rm_matches)) => {
            let name = rm_matches.value_of("name");
            let group = rm_matches.value_of("group");
            cmd::rm(name, group)
        }
        ("list", Some(list_matches)) => {
            let group = list_matches.value_of("group");
            if list_matches.value_of("all").is_some() {
                cmd::list_all()
            } else {
                cmd::list(group)
            }
        }
        ("", None) => {
            let name = matches.value_of("WAYPOINT").unwrap();
            cmd::tele(name)
        },
        _ => unreachable!(),
    }
}
