use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 10,
    title: "Elves Look, Elves Say",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    solve(input, 40)
}

fn solve(input: &str, iterations: u8) -> Solution {
    let mut sequence = input
        .chars()
        .map(|c| c.to_digit(10).expect("Input should only have digits"))
        .collect::<Vec<_>>();

    for _ in 0..iterations {
        sequence = look_and_say(sequence);
    }

    Solution::USize(sequence.len())
}

fn look_and_say(sequence: Vec<u32>) -> Vec<u32> {
    // A new sequence can never be more than twice as long as the previous sequence (worst-case
    // scenario is there are no consecutive numbers, so every number gets a 1 added before it),
    // which gives a useful upper bound for capacity.
    let mut new_sequence = Vec::with_capacity(sequence.len() * 2);
    let mut last_number = None;
    let mut same_number_length = 1;

    for number in sequence {
        if let Some(some_last_number) = last_number {
            if number == some_last_number {
                same_number_length += 1;
            } else {
                // Each time the number changes, add the last run of same numbers and reset the last
                // number and same number length.
                new_sequence.push(same_number_length);
                new_sequence.push(some_last_number);
                last_number = Some(number);
                same_number_length = 1;
            }
        } else {
            last_number = Some(number);
        }
    }

    // At the end of the sequence, add the last run of same numbers.
    new_sequence.push(same_number_length);
    new_sequence.push(last_number.expect("The provided sequence shouldn't be empty"));

    new_sequence
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve("1", 5), Solution::U8(6));
    }
}
