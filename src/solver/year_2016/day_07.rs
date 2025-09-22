use crate::solver::{AdventOfCode, Solution};
use std::collections::VecDeque;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2016,
    day: 7,
    title: "Internet Protocol Version 7",
    part_solvers: &[solve_1, solve_2],
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

    Solution::U16(ip_supporting_tls_count)
}

fn solve_2(input: &str) -> Solution {
    let mut ip_supporting_ssl_count = 0;

    for line in input.lines() {
        let mut in_hypernet_sequence = false;
        // ABAs are found outside hypernet sequences, BABs are found inside hypernet sequences.
        let mut abas = Vec::new();
        let mut babs = Vec::new();
        let mut prev_chars = VecDeque::with_capacity(3);

        for char in line.chars() {
            if matches!(char, '[' | ']') {
                prev_chars.clear();
                in_hypernet_sequence = !in_hypernet_sequence;
            } else {
                if prev_chars.len() == 3 {
                    prev_chars.pop_front();
                }
                prev_chars.push_back(char);

                // If there are three previous characters, check if they form an ABA.
                if prev_chars.len() == 3
                    && prev_chars[0] == prev_chars[2]
                    && prev_chars[0] != prev_chars[1]
                {
                    let new_aba = Aba {
                        first_char: prev_chars[0],
                        second_char: prev_chars[1],
                    };

                    // Check if this new ABA corresponds to any previously found BAB. If in a
                    // hypernet sequence, this "new ABA" is actually a "new BAB", so instead check
                    // if it corresponds to any previously found ABA. If any correspondence is
                    // found, then this IP supports SSL, so add 1 to the count and continue to the
                    // next IP.
                    if in_hypernet_sequence {
                        if abas.iter().any(|aba| new_aba.corresponds(aba)) {
                            ip_supporting_ssl_count += 1;
                            break;
                        }
                        babs.push(new_aba);
                    } else {
                        if babs.iter().any(|bab| new_aba.corresponds(bab)) {
                            ip_supporting_ssl_count += 1;
                            break;
                        }
                        abas.push(new_aba);
                    }
                }
            }
        }
    }

    Solution::U16(ip_supporting_ssl_count)
}

struct Aba {
    first_char: char,
    second_char: char,
}

impl Aba {
    // Returns true if other is the corresponding BAB to this ABA.
    fn corresponds(&self, other: &Aba) -> bool {
        self.first_char == other.second_char && self.second_char == other.first_char
    }
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

    #[test]
    fn example2_1() {
        assert_eq!(solve_2("aba[bab]xyz"), Solution::U8(1));
    }
    #[test]
    fn example2_2() {
        assert_eq!(solve_2("xyx[xyx]xyx"), Solution::U8(0));
    }
    #[test]
    fn example2_3() {
        assert_eq!(solve_2("aaa[kek]eke"), Solution::U8(1));
    }
    #[test]
    fn example2_4() {
        assert_eq!(solve_2("zazbz[bzb]cdb"), Solution::U8(1));
    }
}
