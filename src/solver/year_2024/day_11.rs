use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 11,
    title: "Plutonian Pebbles",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    solve(input, 25)
}

// Part 2 currently would take an enormous amount of time to run if it attempted to perform all 75
// iterations, so it is intentionally set to just 25 for now to avoid blocking a run of all solvers.
fn solve_2(input: &str) -> Solution {
    solve(input, 25)
}

// My code for part 1 already took several seconds, and part 2 requires significantly more numbers
// which my code couldn't handle, so I need to improve the efficiency of my code. My first attempt
// was to change my data structure, so that instead of just being a vector of numbers, it's a vector
// of Stone enums that themselves are either a number or a reference to two other Stones. This way,
// when a stone splits, I can just change create two new stones and change the enum of the current
// stone to point to the new stones, avoiding the need to shift all elements in the vector over by
// one. This was an enormous improvement, increasing the speed of part 1 by roughly a factor of 20.
// However, this code still didn't solve part 2 after running for 10 minutes, so there must still be
// other improvements I'm missing that can speed up this code significantly further.
fn solve(input: &str, iterations: u32) -> Solution {
    #[derive(Debug)]
    enum Stone {
        Single(u64),
        Split(Box<Stone>, Box<Stone>),
    }
    impl Stone {
        fn blink(&mut self) {
            match self {
                Stone::Single(value) => {
                    // Set the stone to 1 if it has the value 0.
                    if *value == 0 {
                        *value = 1;
                    } else {
                        let digits = num_digits(*value);
                        // Split the stone if it has an even number of digits.
                        if digits % 2 == 0 {
                            let (left_stone, right_stone) = split_digits(*value, digits);
                            *self = Stone::Split(
                                Box::new(Stone::Single(left_stone)),
                                Box::new(Stone::Single(right_stone)),
                            );
                        } else {
                            // Otherwise multiply the stone by 2024.
                            *value *= 2024;
                        }
                    }
                }
                // If this stone has split, recursively call blink on the resulting stones.
                Stone::Split(left_stone, right_stone) => {
                    left_stone.blink();
                    right_stone.blink();
                }
            }
        }

        // Recursively count how many individual stones are within this stone which may have split.
        fn len(&self) -> usize {
            match self {
                Stone::Single(_) => 1,
                Stone::Split(left_stone, right_stone) => left_stone.len() + right_stone.len(),
            }
        }
    }

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

    // Get the initial stones.
    let mut stones = input
        .split_whitespace()
        .map(|value| {
            Stone::Single(
                value
                    .parse::<u64>()
                    .expect("Input should only contain numbers"),
            )
        })
        .collect::<Vec<_>>();

    // Update the stones for the specified number of blinks.
    for _ in 0..iterations {
        for stone in &mut stones {
            stone.blink();
        }
    }

    // Count the stones.
    let stones_len = stones.iter().fold(0, |len, stone| len + stone.len());
    Solution::USize(stones_len)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve("0 1 10 99 999", 1), Solution::U8(7));
    }
    #[test]
    fn example1_2() {
        assert_eq!(solve("125 17", 6), Solution::U8(22));
    }
    #[test]
    fn example1_3() {
        assert_eq!(solve("125 17", 25), Solution::U16(55312));
    }
}
