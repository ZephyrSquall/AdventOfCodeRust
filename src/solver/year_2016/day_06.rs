use crate::solver::{Solution, Solver};
use rustc_hash::FxHashMap;

pub const SOLVER: Solver = Solver {
    year: 2016,
    day: 6,
    title: "Signals and Noise",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
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

    // Map each hash map in letter_counts to its most frequent letter, then collect the letters into
    // a String.
    let corrected_message = letter_counts
        .into_iter()
        .map(|letter_count_map| {
            letter_count_map
                .into_iter()
                .max_by_key(|letter_count| letter_count.1)
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
            Solution::Str("easter")
        );
    }
}
