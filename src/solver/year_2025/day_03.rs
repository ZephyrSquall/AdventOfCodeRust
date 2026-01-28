use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2025,
    day: 3,
    title: "Lobby",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    solve(input, 2)
}

fn solve_2(input: &str) -> Solution {
    solve(input, 12)
}

fn solve(input: &str, batteries_to_select: usize) -> Solution {
    let mut total_joltage = 0;

    for line in input.lines() {
        let joltage_ratings = line
            .chars()
            .map(|character| {
                u64::from(
                    character
                        .to_digit(10)
                        .expect("Character should be a number"),
                )
            })
            .collect::<Vec<_>>();

        let mut selected_joltages = Vec::with_capacity(batteries_to_select);
        // When a battery is selected, the starting index is set to one plus that battery's index,
        // to ensure the next battery selected comes after it.
        let mut starting_index = 0;

        for remaining_batteries in (0..batteries_to_select).rev() {
            let (index, joltage_rating) = joltage_ratings
                .iter()
                .enumerate()
                // Do not consider batteries too close to the end such that there would not be
                // enough batteries left to choose from for all the remaining batteries.
                .take(joltage_ratings.len() - remaining_batteries)
                .skip(starting_index)
                // max_by_key returns the last occurrence of the maximum value. We want the first
                // occurrence, so reverse the iterator first.
                .rev()
                .max_by_key(|(_index, joltage_rating)| *joltage_rating)
                .expect("Battery iterator should have at least one battery");

            selected_joltages.push(joltage_rating);
            starting_index = index + 1;
        }

        // Concatenate all the digits of the selected joltages.
        let output_joltage = selected_joltages
            .iter()
            .copied()
            .copied()
            .reduce(|acc, digit| acc * 10 + digit)
            .expect("digits array should not be empty");

        total_joltage += output_joltage;
    }

    Solution::U64(total_joltage)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
987654321111111
811111111111119
234234234234278
818181911112111"
            ),
            Solution::U16(357)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
987654321111111
811111111111119
234234234234278
818181911112111"
            ),
            Solution::U64(3_121_910_778_619)
        );
    }
}
