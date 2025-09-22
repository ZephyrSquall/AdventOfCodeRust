use crate::solver::{AdventOfCode, Solution};
use std::collections::VecDeque;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2016,
    day: 7,
    title: "Internet Protocol Version 7",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let mut ip_supporting_tls_count = 0;

    for line in input.lines() {
        let mut in_hypernet_sequence = false;
        let mut contains_abba = false;
        let mut prev_chars = VecDeque::with_capacity(4);

        for char in line.chars() {
            if matches!(char, '[' | ']') {
                // When entering or leaving a hypernet sequence, previous characters become
                // irrelevant, so clear the list.
                prev_chars.clear();
                in_hypernet_sequence = !in_hypernet_sequence;
            } else {
                // Track only the previous four characters by removing the first character if there
                // are already four characters.
                if prev_chars.len() == 4 {
                    prev_chars.pop_front();
                }
                prev_chars.push_back(char);

                // If there are four previous characters, check if they form an ABBA.
                if prev_chars.len() == 4
                    && prev_chars[0] == prev_chars[3]
                    && prev_chars[1] == prev_chars[2]
                    && prev_chars[0] != prev_chars[1]
                {
                    if in_hypernet_sequence {
                        // An ABBA in a hypernet sequence invalidates any other ABBAs, so set
                        // contains_abba flag to false to prevent counting this IP and immediately
                        // break.
                        contains_abba = false;
                        break;
                    }
                    contains_abba = true;
                }
            }
        }

        if contains_abba {
            ip_supporting_tls_count += 1;
        }
    }

    Solution::U8(ip_supporting_tls_count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("abba[mnop]qrst"), Solution::U8(1));
    }
    #[test]
    fn example1_2() {
        assert_eq!(solve_1("abcd[bddb]xyyx"), Solution::U8(0));
    }
    #[test]
    fn example1_3() {
        assert_eq!(solve_1("aaaa[qwer]tyui"), Solution::U8(0));
    }
    #[test]
    fn example1_4() {
        assert_eq!(solve_1("ioxxoj[asdfgh]zxcvbn"), Solution::U8(1));
    }
}
