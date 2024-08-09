use std::fmt;

pub struct Solver<'a> {
    pub year: u16,
    pub day: u8,
    pub title: &'a str,
    pub part_solvers: &'a [fn(input: &str) -> Solution],
}

// This enum intentionally has dead code as the unused variants are likely to be used in tests and
// future solvers.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Solution<'a> {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    Str(&'a str),
    String(String),
}

impl<'a> fmt::Display for Solution<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Solution::U8(solution) => solution.fmt(f),
            Solution::U16(solution) => solution.fmt(f),
            Solution::U32(solution) => solution.fmt(f),
            Solution::U64(solution) => solution.fmt(f),
            Solution::U128(solution) => solution.fmt(f),
            Solution::USize(solution) => solution.fmt(f),
            Solution::I8(solution) => solution.fmt(f),
            Solution::I16(solution) => solution.fmt(f),
            Solution::I32(solution) => solution.fmt(f),
            Solution::I64(solution) => solution.fmt(f),
            Solution::I128(solution) => solution.fmt(f),
            Solution::ISize(solution) => solution.fmt(f),
            Solution::Str(solution) => solution.fmt(f),
            Solution::String(solution) => solution.fmt(f),
        }
    }
}

// A solution is only intended to be used for printing in the results table. For this purpose, any
// solutions that convert to the same string are equal. Note that testing for solution equality is
// only intended for unit tests; this functionality isn't required by the runner.
impl<'a> PartialEq for Solution<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

pub mod year_2015;
pub mod year_2017;

pub const SOLVERS: [Solver; 32] = [
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
    year_2015::day_07::SOLVER,
    year_2015::day_06::SOLVER,
    year_2015::day_05::SOLVER,
    year_2015::day_04::SOLVER,
    year_2015::day_03::SOLVER,
    year_2015::day_02::SOLVER,
    year_2015::day_01::SOLVER,
];
