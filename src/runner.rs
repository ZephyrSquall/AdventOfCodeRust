use crate::solver::SOLVERS;
use std::fs;
use std::time::Instant;

const DAY_TITLE: &str = "Day";
const PUZZLE_TITLE: &str = "Puzzle";
const PART_TITLE: &str = "Part";
const SOLUTION_TITLE: &str = "Solution";
const TIMING_TITLE: &str = "Time (ms)";

struct MaxLength {
    day: usize,
    puzzle: usize,
    part: usize,
    solution: usize,
    timing: usize,
}

struct SolutionFormat {
    day: String,
    title: String,
    solutions: Vec<String>,
    times: Vec<String>,
}

pub fn run(config: &[u8]) {
    let (solution_formats, max_length) = run_solvers(config);

    print_results_table(&solution_formats, &max_length);
}

fn run_solvers(config: &[u8]) -> (Vec<SolutionFormat>, MaxLength) {
    let mut solution_formats = Vec::with_capacity(SOLVERS.len());
    let mut max_length = MaxLength {
        day: DAY_TITLE.len(),
        puzzle: PUZZLE_TITLE.len(),
        part: PART_TITLE.len(),
        solution: SOLUTION_TITLE.len(),
        timing: TIMING_TITLE.len(),
    };

    for solver in SOLVERS {
        // Only run the solver if the config specified to run this solver or the config didn't
        // specify any particular solvers.
        if config.is_empty() || config.contains(&solver.day) {
            // Fetch the input strings for each puzzle from the text files under puzzle_inputs.
            // "{:02}" left-pads the day number with a 0 if needed so the width of the number is two
            // (text files for the first 9 days are prefixed with a 0 e.g. "01.txt" so it's sorted
            // properly by file systems).
            let file_path = format!("puzzle_inputs/{:02}.txt", solver.day);
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
                if solution.len() > max_length.solution {
                    max_length.solution = solution.len();
                }
                if time.len() > max_length.timing {
                    max_length.timing = time.len();
                }

                solutions.push(solution);
                times.push(time);
            }

            // Store the string representation of all information to be printed in the results
            // table.
            let solution_format = SolutionFormat {
                day: solver.day.to_string(),
                title: solver.title.to_string(),
                solutions,
                times,
            };

            // Check if the length of any data to be displayed exceeds the current maximum length.
            // If so, update the maximum length.
            if solution_format.day.len() > max_length.day {
                max_length.day = solution_format.day.len();
            }
            if solution_format.title.len() > max_length.puzzle {
                max_length.puzzle = solution_format.title.len();
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
        "╔═{empty:═<day_width$}═╤═{empty:═<puzzle_width$}═╤═{empty:═<part_width$}═╤═{empty:═<solution_width$}═╤═{empty:═<timing_width$}═╗",
        empty = "",
        day_width = max_length.day,
        puzzle_width = max_length.puzzle,
        part_width = max_length.part,
        solution_width = max_length.solution,
        timing_width = max_length.timing
    );
    println!(
        "║ {DAY_TITLE:day_width$} │ {PUZZLE_TITLE:puzzle_width$} │ {PART_TITLE:part_width$} │ {SOLUTION_TITLE:solution_width$} │ {TIMING_TITLE:timing_width$} ║",
        day_width = max_length.day,
        puzzle_width = max_length.puzzle,
        part_width = max_length.part,
        solution_width = max_length.solution,
        timing_width = max_length.timing
    );
    println!(
        "╟─{empty:─<day_width$}─┼─{empty:─<puzzle_width$}─┼─{empty:─<part_width$}─┼─{empty:─<solution_width$}─┼─{empty:─<timing_width$}─╢",
        empty = "",
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
                "║ {empty:day_width$} │ {empty:puzzle_width$} │ {empty:part_width$} │ {empty:solution_width$} │ {empty:timing_width$} ║",
                empty = "",
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
                "║ {day:>day_width$} │ {puzzle:puzzle_width$} │ {part:>part_width$} │ {solution:>solution_width$} │ {timing:>timing_width$} ║",
                day = if index == 0 {&solution_format.day} else {""},
                puzzle = if index == 0 {&solution_format.title} else {""},
                part = (index + 1).to_string(),
                solution = solution_format.solutions[index],
                timing = solution_format.times[index],
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
        "╚═{empty:═<day_width$}═╧═{empty:═<puzzle_width$}═╧═{empty:═<part_width$}═╧═{empty:═<solution_width$}═╧═{empty:═<timing_width$}═╝",
        empty = "",
        day_width = max_length.day,
        puzzle_width = max_length.puzzle,
        part_width = max_length.part,
        solution_width = max_length.solution,
        timing_width = max_length.timing
    );
}
