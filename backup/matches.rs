
fn main() {
  let matches = App::new("tele")
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
      .arg(
          Arg::with_name("WAYPOINT")
              .help("Waypoint to tele to...")
              .index(1)
              .required_unless("add")
              .required_unless("rm")
              .required_unless("list")
      )
      .subcommand(
          SubCommand::with_name("add")
              .about("Add working directory to waypoints with supplied name")
              .arg(
                  Arg::with_name("WAYPOINT")
                  .help("Waypoint to be added")
                  .index(1)
                  .required(true),
              )
              .arg(
                  Arg::with_name("group")
                      .help("Optional waypoint group")
                      .short("g")
                      .long("group")
                      .takes_value(true)
              )
      )
      .subcommand(
          SubCommand::with_name("rm")
              .about("Remove waypoint with supplied name")
              .arg(
                  Arg::with_name("WAYPOINT")
                  .help("Waypoint to be removed")
                  .index(1)
                  .required(true),
              )
      )
      .subcommand(
          SubCommand::with_name("list")
              .about("Print all waypoints"),
      )
      .get_matches();
