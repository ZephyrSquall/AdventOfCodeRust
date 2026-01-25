use crate::solver::{AdventOfCode, Solution};
use std::iter::{once, repeat_n};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2025,
    day: 2,
    title: "Gift Shop",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let mut invalid_id_sum = 0;

    for range in input.split(',') {
        let mut range_iter = range.split('-');
        let mut lower_bound_str = range_iter
            .next()
            .expect("Range should have first number")
            .to_string();
        let mut upper_bound_str = range_iter
            .next()
            .expect("Range should have second number")
            .to_string();

        // Numbers with an odd number of digits cannot be formed from a repeating pair of digits. If
        // the lower_bound has an odd number of digits, set it to a '1' followed by a '0' for each
        // digit it has, which is the smallest number with one additional digit. If the upper_bound
        // as an odd number of digits, set it to all '9' for one less than the number of digits is
        // has, which is the largest number with one less digit.
        let lower_bound_len = lower_bound_str.chars().count();
        let upper_bound_len = upper_bound_str.chars().count();

        if !lower_bound_len.is_multiple_of(2) {
            lower_bound_str = once('1').chain(repeat_n('0', lower_bound_len)).collect();
        }
        if !upper_bound_len.is_multiple_of(2) {
            upper_bound_str = repeat_n('9', upper_bound_len - 1).collect();
        }

        // At this point, the strings are guaranteed to have an even number of digits. Split them
        // half-way to get the two halves of each number. Note that the lower_bound_len and
        // upper_bound_len variables can't be reused as the length of the strings may have changed.
        // (It is assumed no range spans more than one order of magnitude, e.g 97-6403, as this
        // doesn't appear in the puzzle input and this would take much more complicated logic to
        // properly handle.)
        let (lower_bound_first_half_str, lower_bound_second_half_str) =
            lower_bound_str.split_at(lower_bound_str.chars().count() / 2);
        let mut lower_bound_first_half = lower_bound_first_half_str
            .parse()
            .expect("First half of lower bound should be a number");
        let lower_bound_second_half = lower_bound_second_half_str
            .parse()
            .expect("Second half of lower bound should be a number");

        let (upper_bound_first_half_str, upper_bound_second_half_str) =
            upper_bound_str.split_at(upper_bound_str.chars().count() / 2);
        let mut upper_bound_first_half = upper_bound_first_half_str
            .parse()
            .expect("First half of upper bound should be a number");
        let upper_bound_second_half = upper_bound_second_half_str
            .parse()
            .expect("Second half of upper bound should be a number");

        // When iterating through all possible numbers in the range, each time the first half of the
        // number goes up by 1, the second half of the number has cycled through every possible
        // number it can be. Exactly one of all of those possible numbers will be the same as the
        // first half of the number. This means that for every time the first half of the number
        // goes up by 1, there is exactly one number that consists of a repeated pair of digits. The
        // only exceptions are the very first or very last number in the range as these are the only
        // numbers where the second half of the number doesn't go through every possible value, so
        // adjust the range to exclude these numbers if needed.
        if lower_bound_first_half < lower_bound_second_half {
            lower_bound_first_half += 1;
        }
        if upper_bound_first_half > upper_bound_second_half {
            upper_bound_first_half -= 1;
        }

        // A range where the lower bound is larger than the upper bound yields no elements, so this
        // loop will do nothing in that case.
        for value in lower_bound_first_half..=upper_bound_first_half {
            let invalid_id = concatenate(value, value);
            invalid_id_sum += invalid_id;
        }
    }

    Solution::U64(invalid_id_sum)
}

fn concatenate(a: u64, b: u64) -> u64 {
    // b.ilog10 + 1 gives the number of digits in b. Multiplying a by 10 to the power of this value
    // will shift a over however many digits is needed to fit b.
    a * 10_u64.pow(b.ilog10() + 1) + b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            Solution::U32(1_227_775_554)
        );
    }
}
