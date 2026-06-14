use crate::solver::{AdventOfCode, Solution};
use std::collections::VecDeque;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2023,
    day: 1,
    title: "Trebuchet?!",
    part_solvers: &[solve_1, solve_2],
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

fn solve_2(input: &str) -> Solution {
    fn char_history_to_digit(char_history: &VecDeque<char>) -> Option<u32> {
        if char_history.len() >= 3 {
            if char_history
                // (char_history.len() - 3).. is a range over the last three values in chat_history.
                .range((char_history.len() - 3)..)
                .eq(['o', 'n', 'e'].iter())
            {
                return Some(1);
            }
            if char_history
                .range((char_history.len() - 3)..)
                .eq(['t', 'w', 'o'].iter())
            {
                return Some(2);
            }
            if char_history
                .range((char_history.len() - 3)..)
                .eq(['s', 'i', 'x'].iter())
            {
                return Some(6);
            }
        }
        if char_history.len() >= 4 {
            if char_history
                .range((char_history.len() - 4)..)
                .eq(['f', 'o', 'u', 'r'].iter())
            {
                return Some(4);
            }
            if char_history
                .range((char_history.len() - 4)..)
                .eq(['f', 'i', 'v', 'e'].iter())
            {
                return Some(5);
            }
            if char_history
                .range((char_history.len() - 4)..)
                .eq(['n', 'i', 'n', 'e'].iter())
            {
                return Some(9);
            }
        }
        if char_history.len() >= 5 {
            if char_history
                .range((char_history.len() - 5)..)
                .eq(['t', 'h', 'r', 'e', 'e'].iter())
            {
                return Some(3);
            }
            if char_history
                .range((char_history.len() - 5)..)
                .eq(['s', 'e', 'v', 'e', 'n'].iter())
            {
                return Some(7);
            }
            if char_history
                .range((char_history.len() - 5)..)
                .eq(['e', 'i', 'g', 'h', 't'].iter())
            {
                return Some(8);
            }
        }

        None
    }

    let mut calibration_value_sum = 0;
    for line in input.lines() {
        let mut char_history = VecDeque::with_capacity(5);
        let mut first_digit = None;
        let mut latest_digit = None;
        for char in line.chars() {
            if char.is_ascii_digit() {
                char_history.clear();

                let digit = char
                    .to_digit(10)
                    .expect("char should be digit in is_ascii_digit branch");
                latest_digit = Some(digit);
                // get_or_insert is used solely for its behaviour of inserting only if first_digit
                // is currently None. The return value is ignored.
                first_digit.get_or_insert(digit);
            } else {
                if char_history.len() == 5 {
                    char_history.pop_front();
                }
                char_history.push_back(char);

                if let Some(digit) = char_history_to_digit(&char_history) {
                    latest_digit = Some(digit);
                    first_digit.get_or_insert(digit);
                }
            }
        }

        let calibration_value = concatenate(
            first_digit.expect("Line should have at least one digit"),
            // After checking the whole line, the value remaining in latest_digit is the last digit.
            latest_digit.expect("Line should have at least one digit"),
        );
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

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            Solution::U16(281)
        );
    }
}
