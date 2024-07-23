use config::parse_arguments;
use runner::run;
use std::env::args;

mod config;
mod runner;
mod solver;

fn main() {
    let config = parse_arguments(args());
    run(&config);
}
