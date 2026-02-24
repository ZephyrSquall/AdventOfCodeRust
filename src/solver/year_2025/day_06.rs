use crate::solver::{AdventOfCode, Solution};
use std::str::SplitWhitespace;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2025,
    day: 6,
    title: "Trash Compactor",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let iters = input
        .lines()
        .map(|line| line.split_whitespace())
        .collect::<Vec<_>>();

    let multi_line = MultiLine { iters };
    let mut grand_total = 0;

    // A "math problem" is a vector of strings representing the values from a single column in the
    // puzzle input. The MultiLine iterator directly produces these vectors because of the way it
    // gets one element from every line at a time.
    for math_problem in multi_line {
        grand_total += solve_math_problem(&math_problem);
    }

    Solution::U64(grand_total)
}

// A custom iterator that takes in a vector of &str iterators. This outer iterator iterates over
// every source iterator simultaneously, producing a vector with the next &str from each source
// iterator until any source iterator produces None (in the context of this problem, they should all
// produce None simultaneously). This is effectively zipping the source iterators. In this case, the
// number of iterators that need to be zipped is unknown at compile time, preventing the use of
// std::iter::zip or itertools::multizip (which use fixed-size tuples) for this purpose.
struct MultiLine<'a> {
    iters: Vec<SplitWhitespace<'a>>,
}

impl<'a> Iterator for MultiLine<'a> {
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iters
            .iter_mut()
            .map(std::iter::Iterator::next)
            .collect::<Option<Vec<_>>>()
    }
}

fn solve_math_problem(math_problem: &Vec<&str>) -> u64 {
    // Reverse the iterator so the first element yielded is the operator and all remaining elements
    // are numbers. Addition and multiplication are commutative so reversing the order of numbers is
    // inconsequential.
    let mut math_problem_iter = math_problem.iter().rev();
    let operator = math_problem_iter
        .next()
        .expect("Math problem should contain operator");

    // All remaining strings represent numbers, so reduce them using the appropriate operator.
    if *operator == "+" {
        math_problem_iter.fold(0, |acc, num| {
            acc + num
                .parse::<u64>()
                .expect("Math problem should contain numbers")
        })
    } else {
        math_problem_iter.fold(1, |acc, num| {
            acc * num
                .parse::<u64>()
                .expect("Math problem should contain numbers")
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            ),
            Solution::U32(4_277_556)
        );
    }
}
