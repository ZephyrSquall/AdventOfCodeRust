use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2025,
    day: 5,
    title: "Cafeteria",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let (ranges_str, ingredients_str) = input
        .split_once("\n\n")
        .expect("Input should have a double newline");
    let ranges: Vec<(u64, u64)> = ranges_str
        .lines()
        .map(|range_str| {
            let (lower_bound_str, upper_bound_str) =
                range_str.split_once('-').expect("Range should have a dash");
            let lower_bound = lower_bound_str
                .parse()
                .expect("Lower bound should be a number");
            let upper_bound = upper_bound_str
                .parse()
                .expect("Upper bound should be a number");
            (lower_bound, upper_bound)
        })
        .collect();

    let fresh_count = ingredients_str
        .lines()
        .map(|ingredient_str| {
            ingredient_str
                .parse::<u64>()
                .expect("Ingredient ID should be a number")
        })
        // Filter out ingredient IDs that don't belong in any range, so only fresh ingredients
        // remain.
        .filter(|ingredient| {
            ranges.iter().any(|(lower_bound, upper_bound)| {
                lower_bound <= ingredient && ingredient <= upper_bound
            })
        })
        .count();

    Solution::USize(fresh_count)
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
}
