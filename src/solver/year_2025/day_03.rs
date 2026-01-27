use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2025,
    day: 3,
    title: "Lobby",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let mut total_joltage = 0;
    for line in input.lines() {
        let battery_count = get_battery_iter(line).count();

        let (first_battery_index, first_battery_joltage_rating) = get_battery_iter(line)
            .max_by_key(|(_index, joltage_rating)| *joltage_rating)
            .expect("Battery iterator should have at least one battery");

        // The second battery must be after the first battery, except when the first battery is the
        // one at the end of the bank, in which case the second battery can be any battery except
        // the one at the end of the bank.
        let (second_battery_index, second_battery_joltage_rating) =
            if first_battery_index == battery_count - 1 {
                get_battery_iter(line)
                    .skip(1)
                    .max_by_key(|(_index, joltage_rating)| *joltage_rating)
                    .expect("Battery iterator should have at least two batteries")
            } else {
                get_battery_iter(line)
                    .take(battery_count - first_battery_index - 1)
                    .max_by_key(|(_index, joltage_rating)| *joltage_rating)
                    .expect("Battery iterator should have at least two batteries")
            };

        let joltage = if first_battery_index < second_battery_index {
            concatenate(first_battery_joltage_rating, second_battery_joltage_rating)
        } else {
            concatenate(second_battery_joltage_rating, first_battery_joltage_rating)
        };
        total_joltage += joltage;
    }

    Solution::U32(total_joltage)
}

fn get_battery_iter(line: &str) -> impl Iterator<Item = (usize, u32)> {
    line.char_indices()
        .map(|(index, character)| {
            (
                index,
                character
                    .to_digit(10)
                    .expect("Each character should be a number"),
            )
        })
        // Reversing the iterator is necessary, as max_by_key() will be called on it, which returns
        // the position of the last occurrence of the maximum value, but we require the first
        // occurrence of the maximum value.
        .rev()
}

fn concatenate(a: u32, b: u32) -> u32 {
    // b.ilog10 + 1 gives the number of digits in b. Multiplying a by 10 to the power of this
    // value will shift a over however many digits is needed to fit b.
    a * 10_u32.pow(b.ilog10() + 1) + b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
987654321111111
811111111111119
234234234234278
818181911112111"
            ),
            Solution::U16(357)
        );
    }
}
