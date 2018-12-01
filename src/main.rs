mod days;

extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let matches = App::new("Advent of Code cli")
        .version("1.1")
        .author("Heiko Carrasco <heiko.carrasco@yahoo.com>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("1")
                .about("First day")
                .arg(Arg::with_name("INPUT").required(true))
                .arg(
                    Arg::with_name("two")
                        .short("t")
                        .help("Compute the value of the second part"),
                ),
        ).get_matches();

    match matches.subcommand_name() {
        Some("1") => {
            let submatches = matches.subcommand_matches("1").unwrap();
            let mut input = File::open(submatches.value_of("INPUT").unwrap()).expect("Not found");
            let mut content = String::new();
            input
                .read_to_string(&mut content)
                .expect("Error reading file");
            if submatches.is_present("two") {
                days::first::first_day_part_two(&content);
            } else {
                days::first::first_day(&content);
            }
        }
        _ => panic!("Bug in clap!"),
    }

}
