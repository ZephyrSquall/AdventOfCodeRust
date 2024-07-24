use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2017,
    day: 15,
    title: "Dueling Generators",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    fn generate_next(value: u64, factor: u64) -> u64 {
        (value * factor) % 2_147_483_647
    }

    let (mut generator_a, mut generator_b) = get_generator_starting_values(input);
    let mut judge_count = 0;

    for _ in 0..40_000_000 {
        generator_a = generate_next(generator_a, 16807);
        generator_b = generate_next(generator_b, 48271);

        // 0xffff is an integer whose last 16 bits are 1 and all remaining bits are 0. Performing a
        // bitwise and with this integer zeroes out all bits except the last 16, which can then be
        // compared with another number that performed this operation to only check if these
        // numbers' last 16 bits match.
        if generator_a & 0xffff == generator_b & 0xffff {
            judge_count += 1;
        }
    }

    Solution::U32(judge_count)
}

fn solve_2(input: &str) -> Solution {
    // generate_next now takes a closure as a third argument that returns true when the generator's
    // condition is satisfied.
    fn generate_next<F: Fn(u64) -> bool>(mut value: u64, factor: u64, condition: F) -> u64 {
        loop {
            value = (value * factor) % 2_147_483_647;
            if condition(value) {
                return value;
            }
        }
    }

    let (mut generator_a, mut generator_b) = get_generator_starting_values(input);
    let mut judge_count = 0;

    for _ in 0..5_000_000 {
        generator_a = generate_next(generator_a, 16807, |v| v % 4 == 0);
        generator_b = generate_next(generator_b, 48271, |v| v % 8 == 0);

        if generator_a & 0xffff == generator_b & 0xffff {
            judge_count += 1;
        }
    }

    Solution::U32(judge_count)
}

fn get_generator_starting_values(input: &str) -> (u64, u64) {
    let mut lines = input.lines();

    // Get the next line of the puzzle input, split it into individual words, take the last word,
    // and convert it to an integer.
    let generator_a = lines
        .next()
        .expect("Input should have first line")
        .split_whitespace()
        .next_back()
        .expect("First line should have text")
        .parse()
        .expect("First line should end with valid number");
    let generator_b = lines
        .next()
        .expect("Input should have second line")
        .split_whitespace()
        .next_back()
        .expect("Second line should have text")
        .parse()
        .expect("Second line should end with valid number");

    (generator_a, generator_b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
Generator A starts with 65
Generator B starts with 8921"
            ),
            Solution::U16(588)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
Generator A starts with 65
Generator B starts with 8921"
            ),
            Solution::U16(309)
        );
    }
}
