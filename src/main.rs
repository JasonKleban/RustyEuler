extern crate clap;
extern crate itertools;
extern crate time;
extern crate num;

use clap::{App, Arg};
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};
use time::PreciseTime;

fn main() {
    let matches = App::new("rusty _euler")
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

fn euler1() -> u32 {
    (1..1000).filter(|x| x % 3 == 0 || x % 5 == 0).fold(0, |a, i| a + i)
}

fn euler2() -> u64 {
    let (_, _, ans) = (0..).fold_while((0, 1, 0u64), |nnns, _| {
        let (nn, n, sum_evens) = nnns;
        if 4_000_000 < n {
            Done(nnns)
        } else {
            Continue((n,
                      nn + n,
                      if (nn + n) % 2 == 0 {
                          sum_evens + nn + n
                      } else {
                          sum_evens
                      }))
        }
    });
    ans
}

fn euler3() -> i64 {
    gpf(600_851_475_143i64)
}

fn gpf(cand_composite: i64) -> i64 {
    if cand_composite == 1 {
        1
    }
    else if cand_composite % 2 == 0 {
        cand_composite / 2
    } else {
        num::range_step((cand_composite as f64).sqrt().floor() as i64, 0, -1)
            .map(|x| {
                //println!("{0} % {1} == {2}", cand_composite, x, cand_composite % x);
                x
            })
            .find(|cand_fact| if cand_composite % cand_fact == 0 && gpf(*cand_fact) == 1 {
                true
            } else {
                false
            })
            .unwrap()
    }
}