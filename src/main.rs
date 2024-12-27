use config::{get_solver_predicate, parse_arguments, LABEL_HEADERS};
use puzzle_results_table::create_results_table;
use solver::SOLVERS;
use std::env::args;

mod config;
mod solver;

fn main() {
    let puzzle_dates = parse_arguments(args());
    let solver_predicate = get_solver_predicate(puzzle_dates);

    create_results_table(&LABEL_HEADERS, &SOLVERS, solver_predicate);
}
