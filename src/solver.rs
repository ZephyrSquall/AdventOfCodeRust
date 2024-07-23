use std::fmt;

pub struct Solver<'a> {
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

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20;
pub mod day_21;
pub mod day_22;
pub mod day_23;
pub mod day_24;
pub mod day_25;

pub const SOLVERS: [Solver; 25] = [
    day_01::SOLVER,
    day_02::SOLVER,
    day_03::SOLVER,
    day_04::SOLVER,
    day_05::SOLVER,
    day_06::SOLVER,
    day_07::SOLVER,
    day_08::SOLVER,
    day_09::SOLVER,
    day_10::SOLVER,
    day_11::SOLVER,
    day_12::SOLVER,
    day_13::SOLVER,
    day_14::SOLVER,
    day_15::SOLVER,
    day_16::SOLVER,
    day_17::SOLVER,
    day_18::SOLVER,
    day_19::SOLVER,
    day_20::SOLVER,
    day_21::SOLVER,
    day_22::SOLVER,
    day_23::SOLVER,
    day_24::SOLVER,
    day_25::SOLVER,
];
