use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2023,
    day: 6,
    title: "Wait For It",
    part_solvers: &[solve_1, solve_2],
};

fn get_ways_to_win(time: u64, distance: u64) -> u64 {
    // let:
    // race time (milliseconds) = t
    // distance record (millimeters) = d
    // charge time (milliseconds) = c
    // distance traveled (millimeters) = x
    //
    // distance = speed * time. In this case, speed (in millimeters per millisecond) is equal to
    // charge time, and time is equal to total time minus charge time. Hence: x = c * (t - c)
    //
    // Rearranging this in terms of c (using the quadratic formula) gives:
    // c = (t ± sqrt(t^2 - 4x)) / 2
    //
    // If we want x to be greater than d, this leads to the following inequalities:
    // (t - sqrt(t^2 - 4d)) / 2 < c < (t + sqrt(t^2 - 4d)) / 2
    //
    // We want to find the smallest and largest integer values of c for which these inequalities
    // hold. Then the number of ways we can beat the record is simply the largest c minus the
    // smallest c plus 1 (how many numbers there are between them, including the limits).
    //
    // To avoid using floats and work only with integers, we can rearrange as follows:
    // t - sqrt(t^2 - 4d) < 2c < t + sqrt(t^2 - 4d)
    //
    // The integer square root function. isqrt, rounds the result of sqrt down. The sqrt function is
    // the only part of the above inequality that can produce fractional values, so replacing it
    // with isqrt ensures the above inequality uses only integer values.
    //
    // Because isqrt rounds down, this ensures t + isqrt(t^2 - 4d) produces the greatest possible
    // integer value of 2c that satisfies the inequality 2c < t + sqrt(t^2 - 4d), and that
    // t - isqrt(t^2 - 4d) produces the smallest possible integer value of 2c that satisfies the
    // inequality t - sqrt(t^2 - 4d) < 2c. Then we simply divide 2c by 2, rounding up for the
    // smallest integer c and rounding down for the largest integer c.
    //
    // There is an edge case when the distance traveled exactly matches the distance record. In this
    // case, t^2 - 4d is a square number, and its square root can be computed exactly without
    // rounding. This would cause 2c to exactly match the lower and upper bounds, which we don't
    // want as we're only concerned with beating the distance record, not tying it. Thus, we must
    // insert a check for whether the charge time bounds results in the exact distance record, and
    // exclude these if so (by subtracting 2 from how many numbers are between them). Note that the
    // parabolic nature of the distance traveled means the situation is symmetric so the lower and
    // upper limits of the charge time always produce the same distance, hence only one bound needs
    // to be checked.

    let discriminant_isqrt = (time * time - 4 * distance).isqrt();
    // t - isqrt(t^2 - 4d) / 2
    let first_winning_charge_time = (time - discriminant_isqrt).div_ceil(2);
    // t + isqrt(t^2 - 4d) / 2 (Note by coincidence this matches the definition for the midpoint
    // between two values; this lint is intentionally disabled as using the midpoint function
    // obscures the intent)
    #[allow(clippy::manual_midpoint)]
    let last_winning_charge_time = (time + discriminant_isqrt) / 2;

    // Check if charge time exactly lines up with distance record, subtract 2 if so.
    if first_winning_charge_time * (time - first_winning_charge_time) == distance {
        last_winning_charge_time - first_winning_charge_time - 1
    } else {
        last_winning_charge_time - first_winning_charge_time + 1
    }
}

fn solve_1(input: &str) -> Solution {
    let mut line_iter = input.lines();
    let times = line_iter
        .next()
        .expect("line_iter should have first line")
        .strip_prefix("Time:")
        .expect("Line should start with \"Time:\"")
        .split_ascii_whitespace()
        .map(|time_str| time_str.parse().expect("time_str should be a valid number"))
        .collect::<Vec<_>>();
    let distances = line_iter
        .next()
        .expect("line_iter should have second line")
        .strip_prefix("Distance:")
        .expect("Line should start with \"Distance:\"")
        .split_ascii_whitespace()
        .map(|distance_str| {
            distance_str
                .parse()
                .expect("distance_str should be a valid number")
        })
        .collect::<Vec<_>>();

    let mut ways_to_win_product = 1;
    for (time, distance) in times.into_iter().zip(distances) {
        ways_to_win_product *= get_ways_to_win(time, distance);
    }

    Solution::U64(ways_to_win_product)
}

fn solve_2(input: &str) -> Solution {
    let mut line_iter = input.lines();
    let time = line_iter
        .next()
        .expect("line_iter should have first line")
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .expect("time string should be a valid number after filtering non-digit characters");
    let distance = line_iter
        .next()
        .expect("line_iter should have second line")
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .expect("distance string should be a valid number after filtering non-digit characters");

    Solution::U64(get_ways_to_win(time, distance))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
Time:      7  15   30
Distance:  9  40  200"
            ),
            Solution::U16(288)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
Time:      7  15   30
Distance:  9  40  200"
            ),
            Solution::U32(71503)
        );
    }
}
