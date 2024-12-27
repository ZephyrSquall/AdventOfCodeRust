use crate::solver::{AdventOfCode, Solution};
use rustc_hash::FxHashMap;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2024,
    day: 19,
    title: "Linen Layout",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    // Recursively check all arrangement of towels to see if the pattern can be created from the
    // available towels.
    fn can_pattern_come_from_towels(
        pattern: &[Stripe],
        towels: &[Vec<Stripe>],
        pattern_index: usize,
    ) -> bool {
        for towel in towels {
            // Check if every stripe in the towel matches the corresponding stripe in the pattern.
            if towel.iter().enumerate().all(|(towel_index, towel_stripe)| {
                pattern
                    // The towel might be longer than the pattern, so use get to return a None if
                    // the towel is too long.
                    .get(pattern_index + towel_index)
                    .is_some_and(|pattern_stripe| *pattern_stripe == *towel_stripe)
            }) {
                let new_pattern_index = pattern_index + towel.len();
                // If the new pattern index matches the length of the pattern, the pattern has been
                // matched perfectly, so return true.
                if new_pattern_index == pattern.len() {
                    return true;
                }
                // Otherwise, recursively check more towels to see if an arrangement of towels to
                // the end of the pattern exists.
                if can_pattern_come_from_towels(pattern, towels, new_pattern_index) {
                    return true;
                }
            }
        }

        // No arrangement of towels to the end of the pattern was found, so return false.
        false
    }

    let (towels, pattern_iter) = get_towels_and_pattern_iter(input);

    // For every pattern, check if it can be made with the available towels.
    let mut possible_patterns = 0;
    for pattern in pattern_iter {
        if can_pattern_come_from_towels(&pattern, &towels, 0) {
            possible_patterns += 1;
        }
    }
    Solution::U32(possible_patterns)
}

fn solve_2(input: &str) -> Solution {
    // Recursively check all arrangement of towels to find the total number of valid arrangements.
    fn pattern_arrangements_from_towels(
        pattern: &[Stripe],
        towels: &[Vec<Stripe>],
        arrangements_at_position: &mut FxHashMap<usize, u64>,
        pattern_index: usize,
    ) -> u64 {
        // The number of arrangements possible from a given position of the pattern never changes.
        // To avoid unnecessary recursion, remember the number of arrangements for a position the
        // first time it is calculated, and use this remembered value if it is needed again.
        if let Some(arrangements) = arrangements_at_position.get(&pattern_index) {
            return *arrangements;
        }

        let mut arrangements = 0;
        for towel in towels {
            if towel.iter().enumerate().all(|(towel_index, towel_stripe)| {
                pattern
                    .get(pattern_index + towel_index)
                    .is_some_and(|pattern_stripe| *pattern_stripe == *towel_stripe)
            }) {
                let new_pattern_index = pattern_index + towel.len();
                // If the new pattern index matches the length of the pattern, the pattern has been
                // matched perfectly, so add 1.
                if new_pattern_index == pattern.len() {
                    arrangements += 1;
                } else {
                    // Otherwise, recursively check more towels to see how many valid arrangements
                    // can be made from here, and add them to the total.
                    arrangements += pattern_arrangements_from_towels(
                        pattern,
                        towels,
                        arrangements_at_position,
                        new_pattern_index,
                    );
                }
            }
        }

        // Remember the number of arrangements at this position, and return it.
        arrangements_at_position.insert(pattern_index, arrangements);
        arrangements
    }

    let (towels, pattern_iter) = get_towels_and_pattern_iter(input);

    let mut possible_arrangements = 0;
    for pattern in pattern_iter {
        // Provide an empty hash map to the first function call so it can be used during the
        // recursion to cache values.
        possible_arrangements +=
            pattern_arrangements_from_towels(&pattern, &towels, &mut FxHashMap::default(), 0);
    }
    Solution::U64(possible_arrangements)
}

#[derive(PartialEq)]
enum Stripe {
    White,
    Blue,
    Black,
    Red,
    Green,
}
impl Stripe {
    fn new(letter: char) -> Stripe {
        match letter {
            'w' => Stripe::White,
            'u' => Stripe::Blue,
            'b' => Stripe::Black,
            'r' => Stripe::Red,
            'g' => Stripe::Green,
            _ => panic!("Stripe colour is incorrect"),
        }
    }
}

fn get_towels_and_pattern_iter(
    input: &str,
) -> (
    Vec<Vec<Stripe>>,
    impl Iterator<Item = Vec<Stripe>> + use<'_>,
) {
    let mut towels = Vec::new();
    let mut line_iter = input.lines();
    let towel_iter = line_iter
        .next()
        .expect("Input should have first line")
        .split(", ");

    // Get every available towel.
    for towel_string in towel_iter {
        let towel = towel_string.chars().map(Stripe::new).collect::<Vec<_>>();
        towels.push(towel);
    }

    let pattern_iter = line_iter
        .skip(1)
        .map(|line| line.chars().map(Stripe::new).collect::<Vec<_>>());

    (towels, pattern_iter)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            ),
            Solution::U8(6)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            ),
            Solution::U8(16)
        );
    }
}
