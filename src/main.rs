extern crate clap;
extern crate time;
mod euler1_3;
mod euler4_6;
mod euler7_9;

use clap::{App, Arg};
use time::PreciseTime;
use self::euler1_3::{euler1, euler2, euler3};
use self::euler4_6::{euler4, euler5, euler6};
use self::euler7_9::{euler7, euler8};

fn main() {
    let matches = App::new("rusty_euler")
        .version("0.0.1")
        .author("Jason Kleban")
        .about("Project Euler problems in Rust")
        .arg(Arg::with_name("r")
            .short("r")
            .long("run")
            .takes_value(true)
            .required(true)
            .multiple(true))
        .get_matches();

    for m in matches.values_of("r").unwrap() {
        match m {
            p @ "1" => run(p, euler1),
            p @ "2" => run(p, euler2),
            p @ "3" => run(p, euler3),
            p @ "4" => run(p, euler4),
            p @ "5" => run(p, euler5),
            p @ "6" => run(p, euler6),
            p @ "7" => run(p, euler7),
            p @ "8" => run(p, euler8),
            x => println!("Problem {0} not yet implemented.", x),
        }
    }
}

fn run<T: std::fmt::Display>(name: &str, func: fn() -> T) -> () {
    let start = PreciseTime::now();
    let ans = func();
    let duration = start.to(PreciseTime::now());
    println!("Problem {0}: {1} ({2})", name, ans, duration);
}

