use crate::solver::{Solution, Solver};
use core::str;

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 11,
    title: "Corporate Policy",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    // Using raw ASCII values makes it easier to "increment" the string. 'a' has ASCII value 97 and
    // 'z' has ASCII value 122, so incrementing a string can be done by adding 1 to the ASCII value
    // and wrapping around from 122 to 97 as needed.
    let mut bytes = input.as_bytes().to_vec();

    while !is_valid_password(&bytes) {
        increment_ascii(&mut bytes);
    }

    Solution::String(String::from_utf8(bytes).expect("Bytes should form a valid string"))
}

fn solve_2(input: &str) -> Solution {
    let mut bytes = input.as_bytes().to_vec();

    let mut valid_passwords: u8 = 0;
    while valid_passwords < 2 {
        increment_ascii(&mut bytes);
        if is_valid_password(&bytes) {
            valid_passwords += 1;
        }
    }

    Solution::String(String::from_utf8(bytes).expect("Bytes should form a valid string"))
}

fn is_valid_password(bytes: &[u8]) -> bool {
    let mut has_increasing_straight_of_three = false;
    let mut increasing_straight_length: u8 = 1;
    let mut doubles: u8 = 0;
    let mut previous_byte_was_paired = false;
    let mut previous_byte = None;

    for byte in bytes {
        // These are the ASCII values for 'i', 'o', and 'l', which are disallowed letters.
        if *byte == 105 || *byte == 111 || *byte == 108 {
            return false;
        }

        if let Some(some_previous_byte) = previous_byte {
            if *byte == some_previous_byte + 1 {
                increasing_straight_length += 1;
            } else {
                increasing_straight_length = 1;
            }
            if increasing_straight_length == 3 {
                has_increasing_straight_of_three = true;
            }

            if !previous_byte_was_paired && *byte == some_previous_byte {
                doubles += 1;
                previous_byte_was_paired = true;
            } else {
                previous_byte_was_paired = false;
            }
        }

        previous_byte = Some(*byte);
    }

    has_increasing_straight_of_three && doubles >= 2
}

fn increment_ascii(bytes: &mut [u8]) {
    for byte in bytes.iter_mut().rev() {
        if *byte == 122 {
            *byte = 97;
        } else {
            *byte += 1;
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("abcdefgh"), Solution::Str("abcdffaa"));
    }
    #[test]
    fn example1_2() {
        assert_eq!(solve_1("ghijklmn"), Solution::Str("ghjaabcc"));
    }
}
