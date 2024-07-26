use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 5,
    title: "Doesn't He Have Intern-Elves For This?",
    part_solvers: &[solve_1],
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
}
