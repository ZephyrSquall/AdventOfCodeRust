use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2016,
    day: 5,
    title: "How About a Nice Game of Chess?",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let mut index = 0;
    let mut password = Vec::with_capacity(8);
    loop {
        let mut hash_input = input.to_string();
        hash_input.push_str(&index.to_string());
        let hash = super::super::year_2015::day_04::md5(&hash_input);
        // Checking if the first 5 hexadecimal digits are 0 is equivalent to checking if the first
        // 20 binary digits are 0. This can be easily tested by right-shifting by 108 (128 - 20) to
        // truncate all but the upper 20 bits, and testing if the result is equal to 0.
        if hash >> 108 == 0 {
            let hash_hexadecimal = format!("{hash:0>32x}");
            password.push(
                hash_hexadecimal
                    .chars()
                    .nth(5)
                    .expect("Hashes should have at least 6 characters"),
            );

            if password.len() == 8 {
                break;
            }
        }

        index += 1;
    }

    Solution::String(password.iter().collect::<String>())
}

fn solve_2(input: &str) -> Solution {
    let mut index = 0;
    let mut password = [None; 8];
    loop {
        let mut hash_input = input.to_string();
        hash_input.push_str(&index.to_string());
        let hash = super::super::year_2015::day_04::md5(&hash_input);
        if hash >> 108 == 0 {
            let hash_hexadecimal = format!("{hash:0>32x}");
            let mut hash_hexadecimal_iter = hash_hexadecimal.chars();
            let password_index = hash_hexadecimal_iter
                .nth(5)
                .expect("Hashes should have at least 6 characters")
                .to_digit(16)
                .expect("The sixth character should be a valid hexadecimal digit")
                as usize;
            // Check if the index fits in the password and a character for that index hasn't already
            // been found.
            if password_index < 8 && password[password_index].is_none() {
                password[password_index] = Some(
                    hash_hexadecimal_iter
                        .next()
                        .expect("Hashes should have at least 7 characters"),
                );
                // Stop looping when all password positions are filled.
                if password.iter().all(Option::is_some) {
                    break;
                }
            }
        }

        index += 1;
    }

    Solution::String(
        password
            .iter()
            .map(|character_option| {
                character_option.expect("All password positions should be filled")
            })
            .collect::<String>(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore = "Finding the correct hashes takes about a minute in debug mode"]
    #[test]
    fn example1_1() {
        assert_eq!(solve_1("abc"), Solution::String("18f47a30".to_string()));
    }

    #[ignore = "Finding the correct hashes takes about a minute in debug mode"]
    #[test]
    fn example2_1() {
        assert_eq!(solve_2("abc"), Solution::String("05ace8e3".to_string()));
    }
}
