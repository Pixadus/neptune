// Description: functions to deal with file inputs and CLI arguments.

use clap::{App, Arg, ArgMatches};

// Get all CLI arguments, if they exist. 
pub fn get_args() -> ArgMatches<'static> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .arg(Arg::with_name("INPUT")
                .help("Input image")
                .required(false)
                .index(1))
            .get_matches();
    matches
}