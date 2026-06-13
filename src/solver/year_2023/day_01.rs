use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2023,
    day: 1,
    title: "Trebuchet?!",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let mut calibration_value_sum = 0;
    for line in input.lines() {
        let first_digit = line
            .chars()
            .find(char::is_ascii_digit)
            .expect("Line should have at least one digit")
            .to_digit(10)
            .expect("Found digit should be a digit");
        let last_digit = line
            .chars()
            .rfind(char::is_ascii_digit)
            .expect("Line should have at least one digit")
            .to_digit(10)
            .expect("Found digit should be a digit");

        let calibration_value = concatenate(first_digit, last_digit);
        calibration_value_sum += calibration_value;
    }
    Solution::U32(calibration_value_sum)
}

fn concatenate(a: u32, b: u32) -> u32 {
    a * 10_u32.pow(b.ilog10() + 1) + b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            Solution::U8(142)
        );
    }
}
