use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2017,
    day: 6,
    title: "Memory Reallocation",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    solve(input, false)
}

fn solve_2(input: &str) -> Solution {
    solve(input, true)
}

fn solve(input: &str, get_loop_size: bool) -> Solution {
    // Use the input to initialize the first bank configuration and bank history.
    let mut banks: Box<[u32]> = input
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Error parsing number"))
        .collect();
    let num_banks = banks.len();
    // banks must be cloned so the real banks array can continue to be mutated without changing
    // history.
    let mut banks_history = vec![banks.clone()];

    loop {
        // Find the max value and the index of this value (note: cannot use a iterator with
        // position_max() as when there are multiple elements with the max value, this returns the
        // index of the last max element but the index of the first max element is needed.)
        let mut max_bank = u32::MIN;
        let mut max_bank_index = 0;
        for (index, bank) in banks.iter().enumerate() {
            if *bank > max_bank {
                max_bank = *bank;
                max_bank_index = index;
            }
        }

        // Distribute blocks.
        banks[max_bank_index] = 0;
        while max_bank > 0 {
            max_bank -= 1;
            // modulo operator ensures index wraps around to 0 as needed.
            max_bank_index = (max_bank_index + 1) % num_banks;

            banks[max_bank_index] += 1;
        }

        // If a position is found, then a match was found between the current banks and the history.
        if let Some(loop_start_index) = banks_history.iter().position(|x| *x == banks) {
            // The length of the bank history indicates how many steps have been taken. For part 2,
            // subtract the index of the start of the loop to get the number of steps in the loop.
            if get_loop_size {
                return Solution::USize(banks_history.len() - loop_start_index);
            }
            return Solution::USize(banks_history.len());
        }

        banks_history.push(banks.clone());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("0 2 7 0"), Solution::U8(5));
    }

    #[test]
    fn example2_1() {
        assert_eq!(solve_2("0 2 7 0"), Solution::U8(4));
    }
}
