use crate::runner::PuzzleDate;
use std::env::Args;

//
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
