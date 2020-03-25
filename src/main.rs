#[macro_use]
extern crate clap;

mod cli;
mod cmd;
mod config;
mod table;
mod waypoints;

fn main() {
    let matches = cli::parse_args();
    cli::parse_matches(matches)
}
