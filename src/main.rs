extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("RustyEuler")
        .version("0.0.1")
        .author("Jason Kleban")
        .about("Project Euler problems in Rust")
        .arg(Arg::with_name("r")
            .short("r")
            .long("run")
            .takes_value(true)
            .multiple(true))
        .get_matches();

        match matches.occurrences_of("r")
        {
            1 | _ => println!("Problem 1: {0}", (1..1000).filter(|x| x % 3 == 0 || x % 5 == 0).fold(0, |a, i| a + i))
        }
}
