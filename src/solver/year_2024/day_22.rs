use crate::solver::{Solution, Solver};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 22,
    title: "Monkey Market",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    // Get the sum of the 2000th secret numbers.
    let mut final_secret_number_sum = 0;
    for mut secret_number in input
        .lines()
        .map(|line| line.parse().expect("Line should have a number"))
    {
        for _ in 0..2000 {
            secret_number = get_next_secret_number(secret_number);
        }
        final_secret_number_sum += secret_number;
    }

    Solution::I64(final_secret_number_sum)
}

fn solve_2(input: &str) -> Solution {
    // Use a hash map to store the total number of bananas acquired so far for every possible
    // sequence of four changes.
    let mut bananas_from_pattern = FxHashMap::default();

    for mut secret_number in input
        .lines()
        .map(|line| line.parse().expect("Line should have a number"))
    {
        // changes tracks the previous four changes.
        let mut changes = VecDeque::with_capacity(4);
        // previous_changes tracks every sequence of four changes that has already been added to the
        // bananas_from_pattern map, since the negotiator monkey only sells for the first occurrence
        // of a sequence from each buyer monkey.
        let mut previous_changes = FxHashSet::default();
        for _ in 0..2000 {
            let next_secret_number = get_next_secret_number(secret_number);
            let bananas_from_sale = secret_number % 10;
            let next_bananas_from_sale = next_secret_number % 10;
            let change = next_bananas_from_sale - bananas_from_sale;

            // Add the next change. Remove the oldest change if there are four changes.
            if changes.len() == 4 {
                changes.pop_front();
            }
            changes.push_back(change);

            // If there are four changes and the current sequence of changes has never occurred
            // before for the current buyer monkey, determine how many bananas would be obtained by
            // selling and add it to the total number of bananas obtained from this pattern.
            if changes.len() == 4 && !previous_changes.contains(&changes) {
                bananas_from_pattern
                    .entry(changes.clone())
                    .and_modify(|bananas| *bananas += next_bananas_from_sale)
                    .or_insert(next_bananas_from_sale);
                previous_changes.insert(changes.clone());
            }

            secret_number = next_secret_number;
        }
    }

    // Find the highest number of bananas obtained out of every pattern.
    let best_bananas = bananas_from_pattern
        .values()
        .max()
        .expect("Map should have at least one value");

    Solution::I64(*best_bananas)
}

// Get the next secret number with the provided pseudorandom algorithm.
fn get_next_secret_number(mut a: i64) -> i64 {
    fn mix_and_prune(a: i64, b: i64) -> i64 {
        (a ^ b) % 16_777_216
    }

    let mul_result = a * 64;
    a = mix_and_prune(a, mul_result);
    let div_result = a / 32;
    a = mix_and_prune(a, div_result);
    let mul_result = a * 2048;
    a = mix_and_prune(a, mul_result);

    a
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
1
10
100
2024"
            ),
            Solution::U32(37_327_623)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
1
2
3
2024"
            ),
            Solution::U8(23)
        );
    }
}
