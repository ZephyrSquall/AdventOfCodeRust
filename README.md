# AdventOfCodeRust

My solutions to the Advent of Code puzzles written in Rust.

This repository contains many "solvers", functions that takes a string representing a puzzle input
and returns a Solution. A Solution is an enum that has a variant for every way an integer or a
string can be represented, allowing solvers to return their solution in whatever format is most
convenient. This repository also contains a runner which will execute all of these solvers and
display their results in a table. This table includes the puzzle's year, day, name, solution for
each part, and time taken for each part. The execution time is only measured once, so it should only
be considered a rough guideline.

Running `cargo run --release` will run every solver that has been created. This can be filtered down
to running the solvers for only a specific year by including a year as a parameter, or a specific
day in a year by including a year followed by a day as parameters. Multiple specific years and/or
days can included as long as the parameter list consists only of years and/or year-day pairs. For
example:
- `cargo run --release 2017` runs ever solver for 2017 puzzles.
- `cargo run --release 2024 02` runs only the solver for the 2024 day 2 puzzle.
- `cargo run --release 2017 2024 02` runs every solver for 2017 puzzles as well as the solver for
  the 2024 day 2 puzzle.

For the runners to run correctly, puzzle inputs must be provided. As the creator of Advent of Code
has requested puzzle inputs not be uploaded publicly, they are omitted from this repository. These
puzzle input files should be .txt files placed in the puzzle_inputs folder under the appropriate
year, with a file name that is simply the day number, e.g. the puzzle input for day 2 of 2015 would
be `puzzle_inputs/2015/02.txt` (note that single-digit day numbers have a leading "0").