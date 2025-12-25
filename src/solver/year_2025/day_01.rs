use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2025,
    day: 1,
    title: "Secret Entrance",
    part_solvers: &[solve_1, solve_2],
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

fn solve_2(input: &str) -> Solution {
    let mut dial_position = 50;
    let mut zeroes = 0;
    let mut previously_moved_right = None;

    for line in input.lines() {
        let (letter, number_str) = line.split_at(1);

        let number = number_str
            .parse::<i32>()
            .expect("number_str should be a valid number");

        // Rust's division operator, "/", uses "arithmetic division", which truncates any fractional
        // part. This is not very useful, as it means both positive and negative dividends will
        // result in 0 if the divisor is large enough, making it difficult to count when the dial
        // goes below zero.

        // Euclidean division instead always rounds towards negative infinity (assuming rhs > 0,
        // which is always true here as rhs is always 100). In particular, this means that when
        // dividing by 100, only numbers in the range 0 to 99 return 0 (unlike with arithmetic
        // division where numbers in the range -99 to 99 return 0). This is much more useful, since
        // now dividing by 100 tells us how many multiples of 100 away a number is from being in the
        // range 0 to 99 (e.g. 243 / 100 = 2 indicates 243 is 2 multiples of 100 too large,
        // -3 / 100 = -1 indicates -3 is 1 multiple of 100 too small). Thus to count how many times
        // the dial passed 0, add the absolute value of the euclidean division of the dial's new
        // position by 100. Euclidean remainder must then be used to put the dial position
        // back into the range 0 to 99 so this logic works for the next dial turn.

        // Note that if the dial ends on 0 exactly and then goes back the way it came, there are two
        // edge cases to consider. If the dial came from the left (from a number greater than 0)
        // then goes back to the right, it never leaves the 0 to 99 range, so the euclidean division
        // method fails to count this. Alternatively, if the dial came from the right (from a number
        // approaching 100 and wrapping around to 0) then goes back to the left, it crosses the
        // boundary of the 0 - 99 range twice, but only pointed to 0 once, thus it gets
        // double-counted.

        if letter == "L" {
            if dial_position == 0 && previously_moved_right == Some(true) {
                // Undo the double-count
                zeroes -= 1;
            }
            dial_position -= number;
            previously_moved_right = Some(false);
        } else {
            if dial_position == 0 && previously_moved_right == Some(false) {
                // Count the missed zero
                zeroes += 1;
            }
            dial_position += number;
            previously_moved_right = Some(true);
        }

        zeroes += dial_position.div_euclid(100).abs();
        dial_position = dial_position.rem_euclid(100);
    }

    Solution::I32(zeroes)
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

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
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
            Solution::U8(6)
        );
    }
}
