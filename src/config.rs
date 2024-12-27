use crate::solver::AdventOfCode;
use std::env::Args;

pub struct PuzzleDate {
    pub year: u16,
    pub day: Option<u8>,
}

pub const LABEL_HEADERS: [&str; 4] = ["Year", "Day", "Puzzle", "Part"];

pub fn parse_arguments(mut args: Args) -> Vec<PuzzleDate> {
    // Discard the first argument, which is just the executable path.
    args.next();

    let mut puzzle_dates = Vec::new();
    // Note that arg may refer to a day or a year, but previous_arg can only refer to a year simply
    // because no branch of the following logic stores a day in it.
    let mut previous_arg = None;
    for arg in args {
        let arg = arg.parse::<u16>().expect("Error parsing argument");
        assert!(
            arg <= 25 || arg >= 2015,
            "Provided number is not a valid day nor a valid year. Days must be 25 or less and years must be 2015 or more."
        );
        let is_year = arg >= 2015;

        if let Some(previous_arg_unwrapped) = previous_arg {
            if is_year {
                // Two years in a row, meaning previous arg refers to a whole year.
                puzzle_dates.push(PuzzleDate {
                    year: previous_arg_unwrapped,
                    day: None,
                });
                previous_arg = Some(arg);
            } else {
                // Year followed by a date, meaning in combination these refer to a specific date.
                // The day is confirmed to be 25 or less, so it can always be successfully converted
                // to a u8 without truncation.
                #[allow(clippy::cast_possible_truncation)]
                puzzle_dates.push(PuzzleDate {
                    year: previous_arg_unwrapped,
                    day: Some(arg as u8),
                });
                previous_arg = None;
            }
        } else if is_year {
            // Year on its own, wait for next argument to see if it's referring to a whole year or a
            // specific date.
            previous_arg = Some(arg);
        } else {
            panic!("All day numbers must be preceded by a year number");
        }
    }

    // Now that all args have been checked, make one final check of the previous arg. If it
    // contains a value, then it was not matched with a day and therefore refers to a whole
    // year.
    if let Some(previous_arg_unwrapped) = previous_arg {
        puzzle_dates.push(PuzzleDate {
            year: previous_arg_unwrapped,
            day: None,
        });
    }

    puzzle_dates
}

// If puzzle_dates is empty, returns true on all solvers. Otherwise, returns true for solvers that
// have at least one successful match, either to a year if a year without a day is specified, or to
// a specific day if a year and day pair is specified.
pub fn get_solver_predicate(puzzle_dates: Vec<PuzzleDate>) -> impl Fn(&AdventOfCode) -> bool {
    move |solver: &AdventOfCode| {
        if puzzle_dates.is_empty() {
            return true;
        }
        for puzzle_date in &puzzle_dates {
            if let Some(day) = puzzle_date.day {
                if solver.year == puzzle_date.year && solver.day == day {
                    return true;
                }
            } else if solver.year == puzzle_date.year {
                return true;
            }
        }
        false
    }
}
