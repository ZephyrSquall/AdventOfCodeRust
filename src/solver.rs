use puzzle_results_table::solver::{Result, Solution, Solver};
use std::{fs, time::Instant};

pub struct AdventOfCode {
    pub year: u16,
    pub day: u8,
    pub title: &'static str,
    pub part_solvers: &'static [fn(input: &str) -> Solution],
}

impl Solver for AdventOfCode {
    fn get_row_count(&self) -> usize {
        self.part_solvers.len()
    }

    fn get_labels(&self, row: usize) -> Vec<String> {
        match row {
            0 => vec![
                self.year.to_string(),
                self.day.to_string(),
                self.title.to_string(),
                "1".to_string(),
            ],
            1 => vec![String::new(), String::new(), String::new(), "2".to_string()],
            _ => panic!("Row number too great"),
        }
    }

    fn execute(&self, row: usize) -> Result {
        let file_path = format!("puzzle_inputs/{}/{:02}.txt", self.year, self.day);
        let input = fs::read_to_string(&file_path).expect("Error reading file");
        let part_solver = self.part_solvers[row];

        let start = Instant::now();
        let solution = part_solver(&input);
        let duration = start.elapsed();

        Result { solution, duration }
    }
}

pub mod year_2015;
pub mod year_2016;
pub mod year_2017;
pub mod year_2024;
pub mod year_2025;

pub const SOLVERS: [AdventOfCode; 83] = [
    year_2025::day_02::SOLVER,
    year_2025::day_01::SOLVER,
    year_2024::day_25::SOLVER,
    year_2024::day_24::SOLVER,
    year_2024::day_23::SOLVER,
    year_2024::day_22::SOLVER,
    year_2024::day_21::SOLVER,
    year_2024::day_20::SOLVER,
    year_2024::day_19::SOLVER,
    // Disable this solver for now, as it takes several minutes to run.
    // year_2024::day_18::SOLVER,
    year_2024::day_17::SOLVER,
    // Disable this solver for now, as on the puzzle input, it does so much recursion that it
    // sometimes overflows the stack.
    // year_2024::day_16::SOLVER,
    year_2024::day_15::SOLVER,
    year_2024::day_14::SOLVER,
    year_2024::day_13::SOLVER,
    year_2024::day_12::SOLVER,
    year_2024::day_11::SOLVER,
    year_2024::day_10::SOLVER,
    year_2024::day_09::SOLVER,
    year_2024::day_08::SOLVER,
    year_2024::day_07::SOLVER,
    year_2024::day_06::SOLVER,
    year_2024::day_05::SOLVER,
    year_2024::day_04::SOLVER,
    year_2024::day_03::SOLVER,
    year_2024::day_02::SOLVER,
    year_2024::day_01::SOLVER,
    year_2017::day_25::SOLVER,
    year_2017::day_24::SOLVER,
    year_2017::day_23::SOLVER,
    year_2017::day_22::SOLVER,
    year_2017::day_21::SOLVER,
    year_2017::day_20::SOLVER,
    year_2017::day_19::SOLVER,
    year_2017::day_18::SOLVER,
    year_2017::day_17::SOLVER,
    year_2017::day_16::SOLVER,
    year_2017::day_15::SOLVER,
    year_2017::day_14::SOLVER,
    year_2017::day_13::SOLVER,
    year_2017::day_12::SOLVER,
    year_2017::day_11::SOLVER,
    year_2017::day_10::SOLVER,
    year_2017::day_09::SOLVER,
    year_2017::day_08::SOLVER,
    year_2017::day_07::SOLVER,
    year_2017::day_06::SOLVER,
    year_2017::day_05::SOLVER,
    year_2017::day_04::SOLVER,
    year_2017::day_03::SOLVER,
    year_2017::day_02::SOLVER,
    year_2017::day_01::SOLVER,
    year_2016::day_08::SOLVER,
    year_2016::day_07::SOLVER,
    year_2016::day_06::SOLVER,
    year_2016::day_05::SOLVER,
    year_2016::day_04::SOLVER,
    year_2016::day_03::SOLVER,
    year_2016::day_02::SOLVER,
    year_2016::day_01::SOLVER,
    year_2015::day_25::SOLVER,
    year_2015::day_24::SOLVER,
    year_2015::day_23::SOLVER,
    year_2015::day_22::SOLVER,
    year_2015::day_21::SOLVER,
    year_2015::day_20::SOLVER,
    year_2015::day_19::SOLVER,
    year_2015::day_18::SOLVER,
    year_2015::day_17::SOLVER,
    year_2015::day_16::SOLVER,
    year_2015::day_15::SOLVER,
    year_2015::day_14::SOLVER,
    year_2015::day_13::SOLVER,
    year_2015::day_12::SOLVER,
    year_2015::day_11::SOLVER,
    year_2015::day_10::SOLVER,
    year_2015::day_09::SOLVER,
    year_2015::day_08::SOLVER,
    year_2015::day_07::SOLVER,
    year_2015::day_06::SOLVER,
    year_2015::day_05::SOLVER,
    year_2015::day_04::SOLVER,
    year_2015::day_03::SOLVER,
    year_2015::day_02::SOLVER,
    year_2015::day_01::SOLVER,
];
