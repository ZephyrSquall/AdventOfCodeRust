use crate::solver::{Solution, Solver};
use itertools::Itertools;

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 5,
    title: "Doesn't He Have Intern-Elves For This?",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let mut nice_strings = 0;

    for line in input.lines() {
        let mut vowels = 0;
        let mut has_double_letter = false;
        let mut has_forbidden_string = false;

        let mut char_iter = line.chars().peekable();
        while let Some(character) = char_iter.next() {
            if let Some(next_character) = char_iter.peek() {
                if (character == 'a' && *next_character == 'b')
                    || (character == 'c' && *next_character == 'd')
                    || (character == 'p' && *next_character == 'q')
                    || (character == 'x' && *next_character == 'y')
                {
                    has_forbidden_string = true;
                    break;
                }

                if character == *next_character {
                    has_double_letter = true;
                }
            }

            if character == 'a'
                || character == 'e'
                || character == 'i'
                || character == 'o'
                || character == 'u'
            {
                vowels += 1;
            }
        }

        if has_double_letter && !has_forbidden_string && vowels >= 3 {
            nice_strings += 1;
        }
    }

    Solution::U32(nice_strings)
}

fn solve_2(input: &str) -> Solution {
    let mut nice_strings = 0;

    for line in input.lines() {
        let characters = line.chars().collect::<Vec<_>>();

        let mut has_repeat_after_any = false;
        let mut has_repeated_pair = false;

        let mut char_iter = characters.iter().multipeek();
        while let Some(character) = char_iter.next() {
            if let Some(_middle_character) = char_iter.peek() {
                if let Some(character_after_any) = char_iter.peek() {
                    if character == *character_after_any {
                        has_repeat_after_any = true;
                    }
                }
            }
        }

        let mut character_pairs = characters.windows(2);
        while let Some(first_character_pair) = character_pairs.next() {
            // Cloning the iterator makes it possible to iterate from the current element to the end
            // searching for matches, while leaving the original iterator untouched so this search
            // can be repeated from the next element.
            let mut character_pairs_clone = character_pairs.clone();
            // Immediately call next() once and ignore the value to avoid counting matches that
            // overlap, such as "aaa".
            character_pairs_clone.next();
            for second_character_pair in character_pairs_clone {
                if first_character_pair == second_character_pair {
                    has_repeated_pair = true;
                }
            }
        }

        if has_repeat_after_any && has_repeated_pair {
            nice_strings += 1;
        }
    }

    Solution::U32(nice_strings)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("ugknbfddgicrmopn"), Solution::U8(1));
    }
    #[test]
    fn example1_2() {
        assert_eq!(solve_1("aaa"), Solution::U8(1));
    }
    #[test]
    fn example1_3() {
        assert_eq!(solve_1("jchzalrnumimnmhp"), Solution::U8(0));
    }
    #[test]
    fn example1_4() {
        assert_eq!(solve_1("haegwjzuvuyypxyu"), Solution::U8(0));
    }
    #[test]
    fn example1_5() {
        assert_eq!(solve_1("dvszwmarrgswjxmb"), Solution::U8(0));
    }

    #[test]
    fn example2_1() {
        assert_eq!(solve_2("qjhvhtzxzqqjkmpb"), Solution::U8(1));
    }
    #[test]
    fn example2_2() {
        assert_eq!(solve_2("xxyxx"), Solution::U8(1));
    }
    #[test]
    fn example2_3() {
        assert_eq!(solve_2("uurcxstgmygtbstg"), Solution::U8(0));
    }
    #[test]
    fn example2_4() {
        assert_eq!(solve_2("ieodomkazucvgmuy"), Solution::U8(0));
    }
}
