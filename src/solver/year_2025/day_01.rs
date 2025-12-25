use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2025,
    day: 1,
    title: "Secret Entrance",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let mut dial_position = 50;
    let mut zeroes = 0;

    for line in input.lines() {
        let (letter, number_str) = line.split_at(1);

        let number = number_str
            .parse::<i32>()
            .expect("number_str should be a valid number");
        if letter == "L" {
            dial_position -= number;
        } else {
            // If not "L", assume it is "R", the only other value that appears in valid puzzle
            // inputs.
            dial_position += number;
        }

        // The real dial wraps around at 100, so it points to 0 any time dial_position is a multiple
        // of 100.
        if dial_position % 100 == 0 {
            zeroes += 1;
        }
    }

    Solution::U32(zeroes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
            ),
            Solution::U8(3)
        );
    }
}
