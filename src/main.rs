mod days;
mod cli;

extern crate clap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let matches = cli::build_cli().get_matches();

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
