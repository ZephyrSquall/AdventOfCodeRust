use crate::solver::{AdventOfCode, Solution};
use rustc_hash::FxHashMap;
use std::cmp::{max_by_key, min_by_key};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2016,
    day: 6,
    title: "Signals and Noise",
    part_solvers: &[solve_1, solve_2],
};

// The only difference between part 1 and part 2 is whether we're looking for the letter with the
// maximum or the minimum count. So create a generic solver that works with any given reducing
// function (the closure provided to the .reduce() method of an iterator), and provide the specific
// reducing function for each specific part solver. Letter counts come from a hash map of char keys
// (the letter) to u32 values (how many of that letter was counted), so they have the type (char,
// u32).
fn solve_1(input: &str) -> Solution {
    fn reducing_function(acc: (char, u32), letter_count: (char, u32)) -> (char, u32) {
        max_by_key(acc, letter_count, |letter_count| letter_count.1)
    }
    solve(input, reducing_function)
}

fn solve_2(input: &str) -> Solution {
    fn reducing_function(acc: (char, u32), letter_count: (char, u32)) -> (char, u32) {
        min_by_key(acc, letter_count, |letter_count| letter_count.1)
    }
    solve(input, reducing_function)
}

fn solve<F: FnMut((char, u32), (char, u32)) -> (char, u32)>(
    input: &str,
    mut reducing_function: F,
) -> Solution {
    // Check the length of the message, then set up a vector of hash maps to track how many times
    // each letter occurs in each position.
    let message_len = input
        .lines()
        .next()
        .expect("Input should have first line")
        .chars()
        .count();
    let mut letter_counts = vec![FxHashMap::default(); message_len];

    // Count the letters in each position.
    for line in input.lines() {
        for (index, letter) in line.chars().enumerate() {
            letter_counts[index]
                .entry(letter)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    // Map each hash map in letter_counts to its most or least frequent letter according to the
    // provided reducing function, then collect the letters into a String.
    let corrected_message = letter_counts
        .into_iter()
        .map(|letter_count_map| {
            letter_count_map
                .into_iter()
                .reduce(&mut reducing_function)
                .expect("Each letter position should have a count for at least one letter")
                .0
        })
        .collect::<String>();

    Solution::String(corrected_message)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"
            ),
            Solution::String("easter".to_string())
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"
            ),
            Solution::String("advent".to_string())
        );
    }
}
