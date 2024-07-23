use super::{Solution, Solver};
use atoi::ascii_to_digit;

pub const SOLVER: Solver = Solver {
    day: 1,
    title: "Inverse Captcha",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let input = input.as_bytes();
    let mut checksum: u32 = 0;

    // Use windows to access every pair of consecutive characters.
    for character_window in input.windows(2) {
        if character_window[0] == character_window[1] {
            let digit = ascii_to_digit::<u32>(character_window[0]).expect("Should decode to digit");
            checksum += digit;
        }
    }

    // Account for wrap-around by comparing the first and last elements.
    if input[0] == input[input.len() - 1] {
        let digit = ascii_to_digit::<u32>(input[0]).expect("Should decode to digit");
        checksum += digit;
    }

    Solution::U32(checksum)
}

fn solve_2(input: &str) -> Solution {
    let input = input.as_bytes();
    let mut checksum: u32 = 0;

    // Note that we only have to check half of the input digits for matches, as every value in the
    // second half will match again to its corresponding digit.
    let half_len = input.len() / 2;

    for i in 0..half_len {
        if input[i] == input[i + half_len] {
            let digit = ascii_to_digit::<u32>(input[i]).expect("Should decode to digit");
            checksum += digit;
        }
    }

    // Double the checksum to account for the skipped second half of the inputs.
    checksum *= 2;

    Solution::U32(checksum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("1122"), Solution::U8(3));
    }
    #[test]
    fn example1_2() {
        assert_eq!(solve_1("1111"), Solution::U8(4));
    }
    #[test]
    fn example1_3() {
        assert_eq!(solve_1("1234"), Solution::U8(0));
    }
    #[test]
    fn example1_4() {
        assert_eq!(solve_1("91212129"), Solution::U8(9));
    }

    #[test]
    fn example2_1() {
        assert_eq!(solve_2("1212"), Solution::U8(6));
    }
    #[test]
    fn example2_2() {
        assert_eq!(solve_2("1221"), Solution::U8(0));
    }
    #[test]
    fn example2_3() {
        assert_eq!(solve_2("123425"), Solution::U8(4));
    }
    #[test]
    fn example2_4() {
        assert_eq!(solve_2("123123"), Solution::U8(12));
    }
    #[test]
    fn example2_5() {
        assert_eq!(solve_2("12131415"), Solution::U8(4));
    }
}
