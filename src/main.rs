#[macro_use]
extern crate clap;

mod cli;
mod cmd;
mod filesystem;
mod table;
mod waypoints;

pub(crate) const TELE: kettle::App = kettle::app("tele", None);

fn main() {
    let matches = cli::parse_args();
    cli::parse_matches(matches)
}
