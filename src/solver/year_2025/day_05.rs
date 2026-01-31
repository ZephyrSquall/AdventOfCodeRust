use crate::solver::{AdventOfCode, Solution};
use std::cmp::{max, min};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2025,
    day: 5,
    title: "Cafeteria",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let (ranges_str, ingredients_str) = input
        .split_once("\n\n")
        .expect("Input should have a double newline");
    let ranges: Vec<Range> = ranges_str.lines().map(get_range).collect();

    let fresh_count = ingredients_str
        .lines()
        .map(|ingredient_str| {
            ingredient_str
                .parse::<u64>()
                .expect("Ingredient ID should be a number")
        })
        // Filter out ingredient IDs that don't belong in any range, so only fresh ingredients
        // remain.
        .filter(|ingredient| ranges.iter().any(|range| range.contains(*ingredient)))
        .count();

    Solution::USize(fresh_count)
}

fn solve_2(input: &str) -> Solution {
    let ranges_str = input
        .split("\n\n")
        .next()
        .expect("Split should give at least one string");

    let mut ranges: Vec<Range> = Vec::new();

    for range_str in ranges_str.lines() {
        let mut new_range = get_range(range_str);

        // Before adding new_range to ranges, merge it with any other ranges it overlaps with, and
        // remove those other ranges. Check ranges in reverse order so that a range getting removed
        // doesn't affect the index of any range yet to be checked. Because this procedure is done
        // with every new_range before adding it, every range in ranges is guaranteed not to
        // overlap.
        for index in (0..ranges.len()).rev() {
            if new_range.merge(&ranges[index]) {
                ranges.swap_remove(index);
            }
        }

        ranges.push(new_range);
    }

    // The amount of fresh IDs contained in a range is inclusive of both the upper and lower bounds,
    // so this is determined by upper - lower + 1. This means 1 will be added for every range,
    // ultimately resulting in ranges.len() being added to the final count. To avoid needlessly
    // adding 1 many times, simply start the count with ranges.len() and only add upper - lower for
    // each range.
    let fresh_count = ranges.iter().fold(ranges.len() as u64, |acc, range| {
        acc + (range.upper - range.lower)
    });

    Solution::U64(fresh_count)
}

struct Range {
    lower: u64,
    upper: u64,
}
impl Range {
    // Returns whether value is inside self
    fn contains(&self, value: u64) -> bool {
        self.lower <= value && value <= self.upper
    }

    // Returns whether self overlaps with other
    fn overlaps(&self, other: &Range) -> bool {
        // If two ranges don't overlap, one of two things must be true: The lower bound of the first
        // range is higher than the upper bound of the second range, or the upper bound of the first
        // range is smaller than the lower bound of the second range. If neither of these are true,
        // then the ranges must therefore overlap.
        !(self.lower > other.upper || self.upper < other.lower)
    }

    // Merges self with other if they overlap, adjusting the bounds of self. Returns a bool
    // indicating whether a merge occurred.
    fn merge(&mut self, other: &Range) -> bool {
        if self.overlaps(other) {
            self.lower = min(self.lower, other.lower);
            self.upper = max(self.upper, other.upper);
            true
        } else {
            false
        }
    }
}

fn get_range(range_str: &str) -> Range {
    let (lower_bound_str, upper_bound_str) =
        range_str.split_once('-').expect("Range should have a dash");
    let lower_bound = lower_bound_str
        .parse()
        .expect("Lower bound should be a number");
    let upper_bound = upper_bound_str
        .parse()
        .expect("Upper bound should be a number");
    Range {
        lower: lower_bound,
        upper: upper_bound,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32"
            ),
            Solution::U8(3)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32"
            ),
            Solution::U8(14)
        );
    }
}
