use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 20,
    title: "Infinite Elves and Infinite Houses",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let target = input.parse::<usize>().expect("Input should be a number");

    // A house will always be visited by the elf whose number matches that house, and that elf will
    // deliver a number of presents equal to 10 times its number, therefore a house is guaranteed to
    // get at least 10 times its number in presents. This means the first house to receive the
    // target number of presents is guaranteed to be before the target number divided by 10.
    let house_limit = target / 10;

    // Create an array that will store a running total of the number of presents delivered to each
    // house (i.e. num_presents_at_house[648] stores the number of presents at house 648). All
    // houses have no presents at the beginning so initialize each value to 0. This needs to use a
    // Box to allocate on the heap, partly because the exact size isn't known at compile time and
    // partly because millions of ints would overflow the stack.
    let mut num_presents_at_house = vec![0; house_limit].into_boxed_slice();

    // Iterate over each elf's journey until the house limit is reached (elves with a greater number
    // will never visit a house whose number is below the house limit).
    for elf_number in 1..house_limit {
        // Each elf starts at the house with its number.
        let mut house_number = elf_number;
        // Technically each elf delivers presents forever, but we can stop at the house limit
        // because the puzzle solution is guaranteed to be reached before this house.
        while house_number < house_limit {
            // Each elf delivers presents equal to 10 times its number to the house it visits.
            num_presents_at_house[house_number] += elf_number * 10;
            // Then the elf moves to the next house that is a multiple of their number.
            house_number += elf_number;
        }
    }

    // All presents have now been delivered to houses under the house limit. Find the first house
    // that got the target number of presents.
    for (house_number, presents) in num_presents_at_house.iter().enumerate() {
        if *presents >= target {
            return Solution::USize(house_number);
        }
    }
    panic!("Should have found a house with at least as many presents as the target number")
}

fn solve_2(input: &str) -> Solution {
    let target = input.parse::<usize>().expect("Input should be a number");

    let house_limit = target / 11;
    let mut num_presents_at_house = vec![0; house_limit].into_boxed_slice();

    for elf_number in 1..house_limit {
        let mut house_number = elf_number;
        let mut houses_visited: u8 = 1;
        // Elves no longer deliver presents infinitely, so stop after they deliver their 50th
        // present (or after they reach the house limit).
        while house_number < house_limit && houses_visited <= 50 {
            num_presents_at_house[house_number] += elf_number * 11;
            house_number += elf_number;
            houses_visited += 1;
        }
    }

    for (house_number, presents) in num_presents_at_house.iter().enumerate() {
        if *presents >= target {
            return Solution::USize(house_number);
        }
    }
    panic!("Should have found a house with at least as many presents as the target number")
}

// The puzzle description provides no examples for this puzzle.
