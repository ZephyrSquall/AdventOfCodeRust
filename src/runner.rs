use crate::solver::{Solver, SOLVERS};
use std::fs;
use std::time::Instant;

const YEAR_TITLE: &str = "Year";
const DAY_TITLE: &str = "Day";
const PUZZLE_TITLE: &str = "Puzzle";
const PART_TITLE: &str = "Part";
const SOLUTION_TITLE: &str = "Solution";
const TIMING_TITLE: &str = "Time (ms)";

pub struct PuzzleDate {
    pub year: u16,
    pub day: Option<u8>,
}

struct MaxLength {
    year: usize,
    day: usize,
    puzzle: usize,
    part: usize,
    solution: usize,
    timing: usize,
}

struct SolutionFormat {
    year: String,
    day: String,
    title: String,
    solutions: Vec<String>,
    times: Vec<String>,
}

pub fn run(config: &[PuzzleDate]) {
    let (solution_formats, max_length) = run_solvers(config);

    print_results_table(&solution_formats, &max_length);
}

fn does_match_date(solver: &Solver, config: &[PuzzleDate]) -> bool {
    if config.is_empty() {
        return true;
    }
    for puzzle_date in config {
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

fn run_solvers(config: &[PuzzleDate]) -> (Vec<SolutionFormat>, MaxLength) {
    let mut solution_formats = Vec::with_capacity(SOLVERS.len());
    let mut max_length = MaxLength {
        year: YEAR_TITLE.chars().count(),
        day: DAY_TITLE.chars().count(),
        puzzle: PUZZLE_TITLE.chars().count(),
        part: PART_TITLE.chars().count(),
        solution: SOLUTION_TITLE.chars().count(),
        timing: TIMING_TITLE.chars().count(),
    };

    for solver in SOLVERS {
        // Only run the solver if the config specified to run this solver or the config didn't
        // specify any particular solvers.
        if does_match_date(&solver, config) {
            // Fetch the input strings for each puzzle from the text files under puzzle_inputs.
            // "{:02}" left-pads the day number with a 0 if needed so the width of the number is two
            // (text files for the first 9 days are prefixed with a 0 e.g. "01.txt" so it's sorted
            // properly by file systems).
            let file_path = format!("puzzle_inputs/{}/{:02}.txt", solver.year, solver.day);
            let input = fs::read_to_string(&file_path).expect("Error reading file");

            let mut solutions = Vec::with_capacity(2);
            let mut times = Vec::with_capacity(2);

            // Run the solvers while measuring their execution time.
            for part_solver in solver.part_solvers {
                let start = Instant::now();
                let solution = part_solver(&input);
                let duration = start.elapsed();
                let time = duration.as_micros();

                let solution = solution.to_string();
                // Pad time strings with zeroes until they are at least four characters long, then
                // insert a decimal point three characters from the end of the string. This way the
                // number of microseconds is converted to a display of milliseconds with a
                // fractional part.
                let mut time = format!("{time:04}");
                time.insert(time.len() - 3, '.');

                // Check if the length of any data to be displayed exceeds the current maximum
                // length. If so, update the maximum length.
                let solution_chars = solution.chars().count();
                if solution_chars > max_length.solution {
                    max_length.solution = solution_chars;
                }
                let time_chars = time.chars().count();
                if time_chars > max_length.timing {
                    max_length.timing = time_chars;
                }

                solutions.push(solution);
                times.push(time);
            }

            // Store the string representation of all information to be printed in the results
            // table.
            let solution_format = SolutionFormat {
                year: solver.year.to_string(),
                day: solver.day.to_string(),
                title: solver.title.to_string(),
                solutions,
                times,
            };

            // Check if the length of any data to be displayed exceeds the current maximum length.
            // If so, update the maximum length.
            let year_chars = solution_format.year.chars().count();
            if year_chars > max_length.year {
                max_length.year = year_chars;
            }
            let day_chars = solution_format.day.chars().count();
            if day_chars > max_length.day {
                max_length.day = day_chars;
            }
            let title_chars = solution_format.title.chars().count();
            if title_chars > max_length.puzzle {
                max_length.puzzle = title_chars;
            }

            solution_formats.push(solution_format);
        }
    }

    (solution_formats, max_length)
}

// This function displays the results table, so it is the only place in this repository where
// printing is intentionally used to create user-facing output.
#[allow(clippy::print_stdout)]
fn print_results_table(solution_formats: &Vec<SolutionFormat>, max_length: &MaxLength) {
    // Generate table header
    println!(
        "╔═{empty:═<year_width$}═╤═{empty:═<day_width$}═╤═{empty:═<puzzle_width$}═╤═{empty:═<part_width$}═╤═{empty:═<solution_width$}═╤═{empty:═<timing_width$}═╗",
        empty = "",
        year_width = max_length.year,
        day_width = max_length.day,
        puzzle_width = max_length.puzzle,
        part_width = max_length.part,
        solution_width = max_length.solution,
        timing_width = max_length.timing
    );
    println!(
        "║ {YEAR_TITLE:year_width$} │ {DAY_TITLE:day_width$} │ {PUZZLE_TITLE:puzzle_width$} │ {PART_TITLE:part_width$} │ {SOLUTION_TITLE:solution_width$} │ {TIMING_TITLE:timing_width$} ║",
        year_width = max_length.year,
        day_width = max_length.day,
        puzzle_width = max_length.puzzle,
        part_width = max_length.part,
        solution_width = max_length.solution,
        timing_width = max_length.timing
    );
    println!(
        "╟─{empty:─<year_width$}─┼─{empty:─<day_width$}─┼─{empty:─<puzzle_width$}─┼─{empty:─<part_width$}─┼─{empty:─<solution_width$}─┼─{empty:─<timing_width$}─╢",
        empty = "",
        year_width = max_length.year,
        day_width = max_length.day,
        puzzle_width = max_length.puzzle,
        part_width = max_length.part,
        solution_width = max_length.solution,
        timing_width = max_length.timing
    );

    // Generate rows for each solution format
    let mut is_first_row = true;
    for solution_format in solution_formats {
        // Skip the empty row if it's the first row.
        if is_first_row {
            is_first_row = false;
        } else {
            println!(
                "║ {empty:year_width$} │ {empty:day_width$} │ {empty:puzzle_width$} │ {empty:part_width$} │ {empty:solution_width$} │ {empty:timing_width$} ║",
                empty = "",
                year_width = max_length.year,
                day_width = max_length.day,
                puzzle_width = max_length.puzzle,
                part_width = max_length.part,
                solution_width = max_length.solution,
                timing_width = max_length.timing
            );
        }
        // Print the rows containing data from the solver. It is assumed that
        // solution_format.solutions.len() and solution_format.times.len() are the same length.
        for index in 0..solution_format.solutions.len() {
            println!(
                "║ {year:>year_width$} │ {day:>day_width$} │ {puzzle:puzzle_width$} │ {part:>part_width$} │ {solution:>solution_width$} │ {timing:>timing_width$} ║",
                year = if index == 0 {&solution_format.year} else {""},
                day = if index == 0 {&solution_format.day} else {""},
                puzzle = if index == 0 {&solution_format.title} else {""},
                part = (index + 1).to_string(),
                solution = solution_format.solutions[index],
                timing = solution_format.times[index],
                year_width = max_length.year,
                day_width = max_length.day,
                puzzle_width = max_length.puzzle,
                part_width = max_length.part,
                solution_width = max_length.solution,
                timing_width = max_length.timing
            );
        }
    }

    // Generate table footer
    println!(
        "╚═{empty:═<year_width$}═╧═{empty:═<day_width$}═╧═{empty:═<puzzle_width$}═╧═{empty:═<part_width$}═╧═{empty:═<solution_width$}═╧═{empty:═<timing_width$}═╝",
        empty = "",
        year_width = max_length.year,
        day_width = max_length.day,
        puzzle_width = max_length.puzzle,
        part_width = max_length.part,
        solution_width = max_length.solution,
        timing_width = max_length.timing
    );
}
