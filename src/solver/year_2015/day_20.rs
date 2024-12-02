use crate::solver::{Solution, Solver};

// Suppress warnings while this solver is excluded.
#[allow(dead_code)]
pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 20,
    title: "Infinite Elves and Infinite Houses",
    part_solvers: &[solve_1],
};

// TODO: Improve this solution. It gives the correct answer, but takes about 12 minutes to run in
// release mode on my setup, so it is clearly not the intended solution.
fn solve_1(input: &str) -> Solution {
    let target = input.parse().expect("Input should be a number");

    let mut house_number = 1;
    loop {
        // A house is visited by each elf whose number is a factor.
        let mut factors = Vec::new();
        for i in 1..=house_number {
            if house_number % i == 0 {
                factors.push(i);
            }
        }

        let presents = factors.into_iter().sum::<u32>() * 10;
        if presents >= target {
            break;
        }

        house_number += 1;
    }

    Solution::U32(house_number)
}

// The puzzle description provides no examples for this puzzle.
