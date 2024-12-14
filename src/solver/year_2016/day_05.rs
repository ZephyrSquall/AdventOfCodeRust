use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2016,
    day: 5,
    title: "How About a Nice Game of Chess?",
    part_solvers: &[solve_1],
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("abc"), Solution::Str("18f47a30"));
    }
}
