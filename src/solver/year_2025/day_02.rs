use crate::solver::{AdventOfCode, Solution};
use std::cmp::{Ordering, max};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2025,
    day: 2,
    title: "Gift Shop",
    part_solvers: &[solve_1, solve_2],
};

// These solutions make extensive use of the property that, in base 10, the number of digits in x is
// given by x.ilog10() + 1.

fn solve_1(input: &str) -> Solution {
    let ranges = get_ranges(input);
    Solution::U64(solve_with_repetitions(&ranges, 2, false))
}

fn solve_2(input: &str) -> Solution {
    let ranges = get_ranges(input);

    // Find the number of digits in the largest number in the puzzle input. This gives an upper
    // bound on how many repetitions an invalid ID can contain.
    let mut max_length = 0;
    for (lower_bound, upper_bound) in &ranges {
        max_length = max(lower_bound.ilog10() + 1, max_length);
        max_length = max(upper_bound.ilog10() + 1, max_length);
    }

    // For every possible number of repetitions, find all invalid IDs with that number of
    // repetitions, and sum them.
    let mut invalid_id_sum = 0;
    for repetitions in 2..=max_length {
        invalid_id_sum += solve_with_repetitions(&ranges, repetitions, true);
    }

    Solution::U64(invalid_id_sum)
}

fn get_ranges(input: &str) -> Vec<(u64, u64)> {
    let mut ranges = Vec::new();

    for range_str in input.split(',') {
        let (lower_bound_str, upper_bound_str) = range_str
            .split_once('-')
            .expect("Range string should contain '-'");
        let lower_bound = lower_bound_str
            .parse()
            .expect("Lower bound should be a number");
        let upper_bound = upper_bound_str
            .parse()
            .expect("Upper bound should be a number");

        ranges.push((lower_bound, upper_bound));
    }

    ranges
}

// In the given ranges, add up all the invalid IDs whose sequence repeats the specified number of
// times. If filter_repeating_sequences is true, exclude invalid IDs which would also have been
// found with a higher repetitions count.
fn solve_with_repetitions(
    ranges: &[(u64, u64)],
    repetitions: u32,
    filter_repeating_sequences: bool,
) -> u64 {
    let mut invalid_id_sum = 0;

    // Iterate over copies of each range, so if a bound is adjusted, it doesn't affect future
    // iterations.
    for (mut lower_bound, mut upper_bound) in ranges.iter().copied() {
        // Numbers can only be split into a sequence that repeats n times if the number of digits in
        // that number is a multiple of n. Interger division followed by multiplication will find
        // the next multiple of n.
        let lower_bound_len = lower_bound.ilog10() + 1;
        let target_lower_bound_len = lower_bound_len.div_ceil(repetitions) * repetitions;
        let upper_bound_len = upper_bound.ilog10() + 1;
        let target_upper_bound_len = upper_bound_len / repetitions * repetitions;

        // If the upper bound became smaller than the lower bound, no numbers in this range can have
        // the specified number of repetitions.
        if target_lower_bound_len > target_upper_bound_len {
            continue;
        }

        // If a bound doesn't have n digits, increase that number to the next smallest number with n
        // digits if it was the lower bound (this would be the next power of 10), or decrease it to
        // the next largest number with n digits if it was the upper bound (this would be the next
        // number 1 less than a power of 10).
        if !lower_bound_len.is_multiple_of(repetitions) {
            lower_bound = 10_u64.pow(target_lower_bound_len - 1);
        }
        if !upper_bound_len.is_multiple_of(repetitions) {
            upper_bound = 10_u64.pow(target_upper_bound_len) - 1;
        }

        // At this point, the lower and upper bounds are guaranteed to have the same number of
        // digits (this isn't strictly guaranteed in general, but it is assumed no range spans more
        // than one order of magnitude, e.g 97-6403, as this doesn't appear in the puzzle input and
        // this would take much more complicated logic to properly handle. If this assumption holds,
        // then there is only a maximum one order of magnitude that can work for any range). Split
        // these bounds into a number of chunks equal to the specified number of repetitions in the
        // invalid ID.
        let sequence_len = target_lower_bound_len / repetitions;
        let mut lower_bound_chunks = chunk_int(lower_bound, sequence_len);
        let mut upper_bound_chunks = chunk_int(upper_bound, sequence_len);

        // When iterating through all possible numbers in the range, each time the first chunk of
        // the number goes up by 1, all other chunks of the number have cycled through every
        // possible number they can be. Exactly one time, all of these other chunks will match the
        // value of the first chunk. This means that for every time the first chunk of the number
        // goes up by 1, there is exactly one number that consists of a repeated pair of digits. The
        // only exceptions are the very first or very last number in the range as these are the only
        // numbers where the second half of the number doesn't go through every possible value, so
        // adjust the range to exclude these numbers if needed.
        //
        // For the lower bound, if the first chunk is smaller than the chunk that comes next, then
        // the very first repeated sequence in the range can't be formed, so skip the lowest value.
        // If the first chunk is greater than the chunk that comes next, then the first repeated
        // sequence can be formed, so include the lowest value. If the first chunk and the chunk
        // that comes next are equal, then whether a repeated sequence can be formed depends on the
        // next chunk, so repeat this check with the next chunk (or include the lowest value if all
        // chunks are equal, as this is precisely what a repeated sequence is). The upper bound
        // follows similar logic, just flipped around.
        for lower_bound_chunk in lower_bound_chunks.iter().skip(1) {
            match lower_bound_chunks[0].cmp(lower_bound_chunk) {
                Ordering::Less => {
                    lower_bound_chunks[0] += 1;
                    break;
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    break;
                }
            }
        }
        for upper_bound_chunk in upper_bound_chunks.iter().skip(1) {
            match upper_bound_chunks[0].cmp(upper_bound_chunk) {
                Ordering::Less => {
                    break;
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    upper_bound_chunks[0] -= 1;
                    break;
                }
            }
        }

        'sequence: for sequence in lower_bound_chunks[0]..=upper_bound_chunks[0] {
            // For part 2, it's possible for the same invalid ID to be obtained multiple ways, e.g.
            // "222222" would be found three separate times, as 6 repetitions of "2", as 3
            // repetitions of "22", and as 2 repetitions of "222". Each invalid ID must only be
            // counted once, so these additional findings must be filtered out. This can be done by
            // checking if the digits of sequence itself contains a repeating sequence of digits; if
            // it is, then it would have already been counted when checking for smaller repetitions
            // of digits.
            //
            // Note that for part 1, these checks must NOT be performed. Since part 1 only checks
            // for the cases of 2 repetitions, it would only find the 2 repetitions of "222", not
            // the 3 repetitions of "22" or the 6 repetitions of "2". Filtering out sequences that
            // have repetition would erroneously cause some invalid IDs to be missed entirely in
            // part 1.
            if filter_repeating_sequences {
                // Check all sequences of digits from length 1 to the length of half the digits in
                // sequence.
                // Clippy suggests to use `sequence.ilog10().div_ceil(2)` instead of
                // `(sequence.ilog10() + 1) / 2` here. This lint is intended to capture manual
                // re-implementations of div_ceil(). However, the intention here is to take the
                // number of digits in sequence, given by `sequence.ilog10() + 1`, and then perform
                // floor division by 2. It is merely coincidence that this is equivalent to
                // performing ceiling division. Actually using div_ceil() here would obscure the
                // real intention.
                #[allow(clippy::manual_div_ceil)]
                for inner_sequence_len in 1..=((sequence.ilog10() + 1) / 2) {
                    let chunks = chunk_int(sequence, inner_sequence_len);
                    // Check if each chunk is the same by checking if every pair of chunks are
                    // equal. If so, the sequence itself consists of repeating sequences of digits,
                    // so skip it.
                    if chunks.windows(2).all(|window| window[0] == window[1]) {
                        continue 'sequence;
                    }
                }
            }

            let invalid_id = repeat_concatenate_self(sequence, repetitions);
            invalid_id_sum += invalid_id;
        }
    }

    invalid_id_sum
}

// Takes a u64 and, assuming it's in base 10, chunks it into a sequence of new u64s that each have
// the specified number of digits from the original number.
// e.g. chunk_int(987654321, 3) returns [987, 654, 321]
fn chunk_int(mut num: u64, size: u32) -> Vec<u64> {
    let num_digits = num.ilog10() + 1;
    let total_chunks = num_digits / size;
    let mut output = Vec::with_capacity(total_chunks as usize);

    for chunk_index in (0..total_chunks).rev() {
        let power_of_ten = 10_u64.pow(chunk_index * size);
        let num_truncated = num / power_of_ten;

        output.push(num_truncated);
        num -= num_truncated * power_of_ten;
    }

    output
}

// Takes a u64 and, assuming it's in base 10, generates a new u64 by concatenating its digits the
// specified number of times.
// e.g. repeat_concatenate_self(321, 3) returns 321321321.
fn repeat_concatenate_self(num: u64, repetitions: u32) -> u64 {
    let mut accumulator = num;

    for _ in 1..repetitions {
        accumulator = accumulator * 10_u64.pow(num.ilog10() + 1) + num;
    }

    accumulator
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

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            Solution::U32(4_174_379_265)
        );
    }
}
