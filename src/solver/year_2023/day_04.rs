use crate::solver::{AdventOfCode, Solution};
use std::collections::BTreeSet;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2023,
    day: 4,
    title: "Scratchcards",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let mut total_points = 0;
    for line in input.lines() {
        let (_, lists) = line
            .split_once(": ")
            .expect("Line should start with \"Card N: \"");
        let (winning_numbers_str, numbers_you_have_str) = lists
            .split_once(" | ")
            .expect("Number lists should be separated by a vertical bar");

        let winning_numbers = winning_numbers_str
            .split_ascii_whitespace()
            .map(|winning_number_str| {
                winning_number_str
                    .parse()
                    .expect("Winning number should be a valid number")
            })
            .collect::<BTreeSet<u8>>();

        let mut points = 0;
        for number_you_have in
            numbers_you_have_str
                .split_ascii_whitespace()
                .map(|number_you_have_str| {
                    number_you_have_str
                        .parse()
                        .expect("Number you have should be a valid number")
                })
        {
            if winning_numbers.contains(&number_you_have) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        total_points += points;
    }

    Solution::U32(total_points)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            Solution::U8(13)
        );
    }
}
