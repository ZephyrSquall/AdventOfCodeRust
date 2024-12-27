use crate::solver::{AdventOfCode, Solution};
use itertools::Itertools;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2024,
    day: 25,
    title: "Code Chronicle",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    // Get the width of each lock and key (it is assumed they're all equal)
    let line_length = input
        .lines()
        .next()
        .expect("Input should have at least one line")
        .chars()
        .count();

    // Get the heights of each pin in every lock and every key.
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    // Remove carriage returns so double newlines can be detected properly.
    let input = input.replace('\r', "");
    for schematic in input.split("\n\n") {
        // The first row is either all '#' or all '.', so only the first character needs to be
        // checked to determine if this is a lock or a key.
        if schematic.starts_with('#') {
            let mut lock_lengths = vec![0; line_length];
            for line in schematic.lines().skip(1) {
                for (column, char) in line.chars().enumerate() {
                    if char == '#' {
                        lock_lengths[column] += 1;
                    }
                }
            }
            locks.push(lock_lengths);
        } else {
            let mut key_lengths = vec![0; line_length];
            // If it's a key, do the same thing as for a lock but reverse the lines first.
            for line in schematic.lines().rev().skip(1) {
                for (column, char) in line.chars().enumerate() {
                    if char == '#' {
                        key_lengths[column] += 1;
                    }
                }
            }
            keys.push(key_lengths);
        }
    }

    // Count all valid lock-key pairs.
    let mut lock_key_pairs = 0;
    // Use the cartesian product to get every possible pair of locks and keys.
    for (lock_lengths, key_lengths) in locks.iter().cartesian_product(&keys) {
        // Check if every pin doesn't exceed the maximum length of 5.
        if lock_lengths
            .iter()
            .zip(key_lengths)
            .all(|(lock_length, key_length)| *lock_length + *key_length <= 5)
        {
            lock_key_pairs += 1;
        }
    }

    Solution::U32(lock_key_pairs)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"
            ),
            Solution::U8(3)
        );
    }
}
