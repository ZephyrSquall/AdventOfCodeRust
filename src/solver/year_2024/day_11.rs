use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 11,
    title: "Plutonian Pebbles",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    // Get the number of digits in a number
    fn num_digits(a: u64) -> u32 {
        a.ilog10() + 1
    }

    // Split a number with an even number of digits into two numbers with the left half and right
    // half of the digits. This takes advantage of how dividing a number (in base 10) by a power of
    // 10 gives that number with fewer digits, and the digits that were lost are in the remainder.
    fn split_digits(a: u64, mut digits: u32) -> (u64, u64) {
        digits /= 2;
        let ten_power = 10_u64.pow(digits);
        (a / ten_power, a % ten_power)
    }

    let mut stones = input
        .split_whitespace()
        .map(|value| {
            value
                .parse::<u64>()
                .expect("Input should only contain numbers")
        })
        .collect::<Vec<_>>();

    for _ in 0..25 {
        // Iterate over stones backwards, so if stones are shifted by the insertion of a new stone,
        // the index of stones that haven't been iterated over yet are unaffected.
        for index in (0..stones.len()).rev() {
            let stone = &mut stones[index];
            if *stone == 0 {
                *stone = 1;
            } else {
                let stone_digits = num_digits(*stone);
                if stone_digits % 2 == 0 {
                    let (left_stone, right_stone) = split_digits(*stone, stone_digits);
                    *stone = left_stone;
                    // The insert has to be after assigning stone a new value so the mutable borrow
                    // on stone is dropped, as a vector can't be mutably borrowed while an element
                    // of it is also mutably borrowed.
                    stones.insert(index + 1, right_stone);
                } else {
                    *stone *= 2024;
                }
            }
        }
    }

    Solution::USize(stones.len())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("125 17"), Solution::U16(55312));
    }
}
