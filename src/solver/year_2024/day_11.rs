use crate::solver::{Solution, Solver};
use rustc_hash::FxHashMap;

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 11,
    title: "Plutonian Pebbles",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    solve(input, 25)
}

fn solve_2(input: &str) -> Solution {
    solve(input, 75)
}

fn solve(input: &str, iterations: usize) -> Solution {
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

    // Get the number of stones that a stone with the given value will have split into after the
    // given number of blinks. First the cache is searched to see if it contains the required
    // number, if not the number is calculated and added to the cache.
    fn get_stone_splits(
        value: u64,
        blinks: usize,
        stone_num_cache: &mut FxHashMap<u64, Vec<u64>>,
    ) -> u64 {
        // stone_after_blinks is a vector arranged such that stone_num_after_blinks[blinks] gives
        // the number of stones that the given stone will split into after that many blinks. If it
        // isn't in the cache, initialize it with an empty vector.
        let stone_num_after_blinks = stone_num_cache
            .entry(value)
            .or_insert(Vec::with_capacity(blinks));

        // If the value is in the cache, return that value.
        if blinks < stone_num_after_blinks.len() {
            return stone_num_after_blinks[blinks];
        }

        // Otherwise, build up to that value. Start from 0 blinks, which trivially leads to 1 stone.
        // For each blink after that, calculate which stone or stones the current stone turns into,
        // and attempt to fetch their value by recursively calling this function, which will also
        // add those stones to the cache. Repeat this process until the required number of blinks
        // are reached.
        for prior_blink in stone_num_after_blinks.len()..=blinks {
            if prior_blink == 0 {
                // stone_num_after_blinks cannot be reused within this loop as it takes a mutable
                // borrow on stone_num_cache and another mutable borrow is required within this loop
                // if get_stone_splits needs to be called recursively. Hence this value is
                // separately obtained with get_mut so all mutable borrows on the cache are
                // immediately dropped after they are used.
                stone_num_cache
                    .get_mut(&value)
                    .expect("Stone value should be in cache")
                    .push(1);
            } else {
                // Get the value of all stones that follow from this stone.
                let mut child_values = Vec::with_capacity(2);
                // Set the stone to 1 if it has the value 0.
                if value == 0 {
                    child_values.push(1);
                } else {
                    let digits = num_digits(value);
                    // Split the stone if it has an even number of digits.
                    if digits % 2 == 0 {
                        let (left_stone, right_stone) = split_digits(value, digits);
                        child_values.push(left_stone);
                        child_values.push(right_stone);
                    } else {
                        // Otherwise multiply the stone by 2024.
                        child_values.push(value * 2024);
                    }
                }

                // The amount of stones after the given number of blinks is equal to the number
                // stones that all child stones would have split into after one less than the given
                // number of blinks. Recursively call this function to find those numbers and add
                // them together.
                let mut stone_num = 0;
                for child_value in child_values {
                    stone_num += get_stone_splits(child_value, prior_blink - 1, stone_num_cache);
                }
                // The cache is always built from 0 to the current value, so pushing will always
                // insert at the correct index.
                stone_num_cache
                    .get_mut(&value)
                    .expect("Stone value should be in cache")
                    .push(stone_num);
            }
        }

        // When the above loop is exited, the number of stones at the given number of blinks has
        // been inserted into the cache, so fetch it and return it.
        stone_num_cache
            .get(&value)
            .expect("Stone value should be in cache")[blinks]
    }

    // Get the initial stones.
    let stone_values = input
        .split_whitespace()
        .map(|value| {
            value
                .parse::<u64>()
                .expect("Input should only contain numbers")
        })
        .collect::<Vec<_>>();

    // Initialize the cache. It's a map indicating how many stones does a stone of a certain value
    // (the map's key) split into after a certain number of blinks (the index of the value vector at
    // that key).
    let mut stone_num_cache: FxHashMap<u64, Vec<u64>> = FxHashMap::default();

    // Count the stones.
    let mut stone_num = 0;
    for stone_value in stone_values {
        stone_num += get_stone_splits(stone_value, iterations, &mut stone_num_cache);
    }
    Solution::U64(stone_num)
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
