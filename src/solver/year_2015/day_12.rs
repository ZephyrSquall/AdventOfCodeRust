use crate::solver::{Solution, Solver};
use std::str::Chars;

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 12,
    title: "JSAbacusFramework.io",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let mut digits = Vec::new();
    let mut is_negative = false;
    let mut sum = 0;

    for character in input.chars() {
        if let Some(digit) = character.to_digit(10) {
            // If the character is a digit, add it to the list of digits.
            digits.push(digit);
        } else if !digits.is_empty() {
            // If the character isn't a digit and there are digits in the list of digits, then a
            // number in the input string has just ended, so convert the list of digits into a
            // number and either add or subtract it from the sum depending on whether it was
            // negative. Then clear the list of digits.
            if is_negative {
                sum -= i64::from(parse_digits(&digits));
                is_negative = false;
            } else {
                sum += i64::from(parse_digits(&digits));
            }
            digits.clear();
        } else {
            // Flag if a minus sign is encountered, to determine if an immediately-following number
            // is negative.
            is_negative = character == '-';
        }
    }

    Solution::I64(sum)
}

fn solve_2(input: &str) -> Solution {
    // A "structure" is either an object or array within the JSON. Which one it is is specified by
    // the "is_object" argument. Both structures read and count numbers in the same way and so can
    // reuse most of their code; the only differences between processing objects and arrays is that
    // only objects care about seeing a "red" string (though both objects and arrays care if a
    // parent object saw a "red" string, hence the has_found_red flag is passed down).
    fn count_numbers_in_structure(
        char_iter: &mut Chars,
        is_object: bool,
        mut has_found_red: bool,
    ) -> i64 {
        let mut digits = Vec::new();
        let mut is_negative = false;
        let mut sum = 0;
        let mut red_letters: u8 = 0;

        loop {
            // If the JSON is correctly formatted, there will be exactly as many opening characters
            // ('{' and '[') as closing characters ('}' and ']'). This means the topmost structure's
            // closing character will be the last character of the string, and hence the loop will
            // break itself on the last character. If char_iter.next() returns None, that means the
            // JSON was not correctly formatted and there were more opening characters than closing
            // characters.
            let character = char_iter.next().expect("Iterator should not be completely consumed before exiting from the topmost structure");

            // Look for numbers and sum them.
            if let Some(digit) = character.to_digit(10) {
                digits.push(digit);
            } else if !digits.is_empty() {
                if is_negative {
                    sum -= i64::from(parse_digits(&digits));
                    is_negative = false;
                } else {
                    sum += i64::from(parse_digits(&digits));
                }
                digits.clear();
            } else {
                is_negative = character == '-';
            }

            // If the JSON is correctly formatted, a '}' will never be encountered while treating
            // the structure as an array and a ']' will never be encountered while treating the
            // structure as an object. Assuming the puzzle input is valid JSON, it's safe to simply
            // close the structure when either of these characters are found.
            if character == '}' || character == ']' {
                break;
            } else if character == '{' {
                // Recursively call this function as needed upon seeing opening characters. This is
                // critical to ensuring the first closing character this structure sees that wasn't
                // consumed by a child structure is matched to this structure.
                sum += count_numbers_in_structure(char_iter, true, has_found_red);
            } else if character == '[' {
                sum += count_numbers_in_structure(char_iter, false, has_found_red);
            }

            if is_object {
                if character == 'r' && red_letters == 0 {
                    red_letters = 1;
                } else if character == 'e' && red_letters == 1 {
                    red_letters = 2;
                } else if character == 'd' && red_letters == 2 {
                    // The word "red" was found in this object, so set a flag to indicate that this
                    // function should return 0 regardless of the calculated sum. Note that the loop
                    // must still continue, recursively calling count_numbers_in_structure as
                    // needed, to ensure all characters in the iterator until the corresponding
                    // closing '}' are consumed.
                    has_found_red = true;
                } else {
                    red_letters = 0;
                }
            }
        }

        if has_found_red {
            0
        } else {
            sum
        }
    }

    let mut char_iter = input.chars();
    let sum = match char_iter
        .next()
        .expect("Input string should have first character")
    {
        '{' => count_numbers_in_structure(&mut char_iter, true, false),
        '[' => count_numbers_in_structure(&mut char_iter, false, false),
        _ => panic!("First character should indicate the start of an object or array"),
    };
    Solution::I64(sum)
}

// Takes an array of digits and converts them into the number formed by concatenating the digits.
fn parse_digits(digits: &[u32]) -> u32 {
    let mut multiplier = 1;
    let mut number = 0;

    for digit in digits.iter().rev() {
        number += digit * multiplier;
        multiplier *= 10;
    }

    number
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("[1,2,3]"), Solution::U8(6));
    }
    #[test]
    fn example1_2() {
        assert_eq!(solve_1("{\"a\":2,\"b\":4}"), Solution::U8(6));
    }
    #[test]
    fn example1_3() {
        assert_eq!(solve_1("[[[3]]]"), Solution::U8(3));
    }
    #[test]
    fn example1_4() {
        assert_eq!(solve_1("{\"a\":{\"b\":4},\"c\":-1}"), Solution::U8(3));
    }
    #[test]
    fn example1_5() {
        assert_eq!(solve_1("{\"a\":[-1,1]}"), Solution::U8(0));
    }
    #[test]
    fn example1_6() {
        assert_eq!(solve_1("[-1,{\"a\":1}]"), Solution::U8(0));
    }
    #[test]
    fn example1_7() {
        assert_eq!(solve_1("[]"), Solution::U8(0));
    }
    #[test]
    fn example1_8() {
        assert_eq!(solve_1("{}"), Solution::U8(0));
    }

    #[test]
    fn example2_1() {
        assert_eq!(solve_2("[1,2,3]"), Solution::U8(6));
    }
    #[test]
    fn example2_2() {
        assert_eq!(solve_2("[1,{\"c\":\"red\",\"b\":2},3]"), Solution::U8(4));
    }
    #[test]
    fn example2_3() {
        assert_eq!(
            solve_2("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"),
            Solution::U8(0)
        );
    }
    #[test]
    fn example2_4() {
        assert_eq!(solve_2("[1,\"red\",5]"), Solution::U8(6));
    }
}
