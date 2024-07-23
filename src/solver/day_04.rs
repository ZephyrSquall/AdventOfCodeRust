use super::{Solution, Solver};
use itertools::Itertools;

pub const SOLVER: Solver = Solver {
    day: 4,
    title: "High-Entropy Passphrases",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    solve(input, false)
}

fn solve_2(input: &str) -> Solution {
    solve(input, true)
}

fn solve(input: &str, test_anagrams: bool) -> Solution {
    let mut valid_passphrase_count: u32 = 0;

    for line in input.lines() {
        // Count every passphrase.
        valid_passphrase_count += 1;

        // Check every combination of words for a match.
        for (word1, word2) in line.split(' ').tuple_combinations::<(&str, &str)>() {
            if word_match(word1, word2, test_anagrams) {
                // Undo counting this passphrase, as it has matching words and is therefore invalid.
                valid_passphrase_count -= 1;
                break;
            }
        }
    }

    Solution::U32(valid_passphrase_count)
}

fn word_match(word1: &str, word2: &str, test_anagrams: bool) -> bool {
    if test_anagrams {
        // Sorts the letters of the words alphabetically (unstable sort used for performance).
        // Anagrams of a word will all sort to the same word, thus strings which are equal after
        // this procedure are anagrams of each other.
        word1
            .chars()
            .sorted_unstable()
            .eq(word2.chars().sorted_unstable())
    } else {
        word1 == word2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("aa bb cc dd ee"), Solution::U8(1));
    }
    #[test]
    fn example1_2() {
        assert_eq!(solve_1("aa bb cc dd aa"), Solution::U8(0));
    }
    #[test]
    fn example1_3() {
        assert_eq!(solve_1("aa bb cc dd aaa"), Solution::U8(1));
    }

    #[test]
    fn example2_1() {
        assert_eq!(solve_2("abcde fghij"), Solution::U8(1));
    }
    #[test]
    fn example2_2() {
        assert_eq!(solve_2("abcde xyz ecdab"), Solution::U8(0));
    }
    #[test]
    fn example2_3() {
        assert_eq!(solve_2("a ab abc abd abf abj"), Solution::U8(1));
    }
    #[test]
    fn example2_4() {
        assert_eq!(solve_2("iiii oiii ooii oooi oooo"), Solution::U8(1));
    }
    #[test]
    fn example2_5() {
        assert_eq!(solve_2("oiii ioii iioi iiio"), Solution::U8(0));
    }
}
