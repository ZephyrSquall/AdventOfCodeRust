use crate::solver::{Solution, AdventOfCode};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2015,
    day: 8,
    title: "Matchsticks",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    // Tracking the absolute number of characters in code and characters in memory is unnecessary,
    // only the difference between them matters.
    let mut character_difference = 0;

    for line in input.lines() {
        // Always add 2 to character_difference for each line to represent the difference caused by
        // the opening and closing quotation marks.
        character_difference += 2;
        let mut is_escaped = false;

        for character in line.chars() {
            if is_escaped {
                match character {
                    // \" and \\ are both 2 characters in code and 1 character in memory, for a
                    // difference of 1.
                    '\"' | '\\' => character_difference += 1,
                    // \x00 ("00" is any two-digit hexadecimal number) is 4 characters in code and 1
                    // character in memory, for a difference of 3. A hexadecimal number can never
                    // include a back-slash, so it's safe to set is_escaped back to false even
                    // though we've only reached the 'x' character in the string so far.
                    'x' => character_difference += 3,
                    _ => panic!("Unrecognized escape sequence"),
                }
                is_escaped = false;
            } else {
                if character == '\\' {
                    is_escaped = true;
                }
                // If a character is not escaped, then it is 1 character in code and 1 character in
                // memory, for a difference of 0.
            }
        }
    }

    Solution::U32(character_difference)
}

fn solve_2(input: &str) -> Solution {
    // Tracking the absolute number of characters in the encoded string code and characters in the
    // original string is unnecessary, only the difference between them matters.
    let mut character_difference = 0;

    for line in input.lines() {
        // Always add 2 to character_difference for each line to represent the difference caused by
        // wrapping the string in another pair of quotation marks.
        character_difference += 2;

        for character in line.chars() {
            // The only characters that need to be escaped are double quotes and backslashes, and
            // doing so requires simply adding an additional backslash which causes a difference of
            // 1 character. All other characters are simply displayed as-is, for a difference of 0
            // characters.
            if character == '\"' || character == '\\' {
                character_difference += 1;
            }
        }
    }

    Solution::U32(character_difference)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        // Input string is: ""
        assert_eq!(solve_1("\"\""), Solution::U8(2));
    }
    #[test]
    fn example1_2() {
        // Input string is: "abc"
        assert_eq!(solve_1("\"abc\""), Solution::U8(2));
    }
    #[test]
    fn example1_3() {
        // Input string is: "aaa\"aaa"
        assert_eq!(solve_1("\"aaa\\\"aaa\""), Solution::U8(3));
    }
    #[test]
    fn example1_4() {
        // Input string is: "\x27"
        assert_eq!(solve_1("\"\\x27\""), Solution::U8(5));
    }

    #[test]
    fn example2_1() {
        // Input string is: ""
        assert_eq!(solve_2("\"\""), Solution::U8(4));
    }
    #[test]
    fn example2_2() {
        // Input string is: "abc"
        assert_eq!(solve_2("\"abc\""), Solution::U8(4));
    }
    #[test]
    fn example2_3() {
        // Input string is: "aaa\"aaa"
        assert_eq!(solve_2("\"aaa\\\"aaa\""), Solution::U8(6));
    }
    #[test]
    fn example2_4() {
        // Input string is: "\x27"
        assert_eq!(solve_2("\"\\x27\""), Solution::U8(5));
    }
}
