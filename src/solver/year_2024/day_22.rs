use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 22,
    title: "Monkey Market",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    // Get the next secret number with the provided pseudorandom algorithm.
    fn next_secret_number(mut a: u64) -> u64 {
        fn mix_and_prune(a: u64, b: u64) -> u64 {
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

    // Get the sum of the 2000th secret numbers.
    let mut final_secret_number_sum = 0;
    for mut secret_number in input
        .lines()
        .map(|line| line.parse().expect("Line should have a number"))
    {
        for _ in 0..2000 {
            secret_number = next_secret_number(secret_number);
        }
        final_secret_number_sum += secret_number;
    }

    Solution::U64(final_secret_number_sum)
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
}
