use crate::solver::{Solution, AdventOfCode};
use itertools::Itertools;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2017,
    day: 2,
    title: "Corruption Checksum",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let mut checksum = 0;

    for line in input.lines() {
        let mut largest_int = i32::MIN;
        let mut smallest_int = i32::MAX;

        for number in line.split_whitespace() {
            let number = number.parse::<i32>().expect("Error parsing number");
            if number > largest_int {
                largest_int = number;
            }
            if number < smallest_int {
                smallest_int = number;
            }
        }

        checksum += largest_int - smallest_int;
    }

    Solution::I32(checksum)
}

fn solve_2(input: &str) -> Solution {
    let mut checksum = 0;

    for line in input.lines() {
        let numbers_iter = line
            .split_whitespace()
            .map(|string| string.parse::<i32>().expect("Error parsing number"));

        // .permutations gives every pair of numbers. Both orderings of each pair of numbers are
        // present, hence it is not necessary to check number[1] % number[0] == 0 for any pair.
        for number in numbers_iter.permutations(2) {
            if number[0] % number[1] == 0 {
                checksum += number[0] / number[1];
                break;
            }
        }
    }

    Solution::I32(checksum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
5 1 9 5
7 5 3
2 4 6 8"
            ),
            Solution::U8(18)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
5 9 2 8
9 4 7 3
3 8 6 5"
            ),
            Solution::U8(9)
        );
    }
}
