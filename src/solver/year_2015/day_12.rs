use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 12,
    title: "JSAbacusFramework.io",
    part_solvers: &[solve_1],
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
}
