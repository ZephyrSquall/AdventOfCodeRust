use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2025,
    day: 6,
    title: "Trash Compactor",
    part_solvers: &[solve_1, solve_2],
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

fn solve_2(input: &str) -> Solution {
    let iters = input
        .lines()
        // split("") with an empty string as the argument will produce an iterator over every
        // individual character of the provided string. Similar to .chars() but the individual
        // characters are &str instead of char. However, split("") also creates an empty string at
        // the start and the end of the iterator. The first empty string in each line is skipped.
        // The last empty string could also be removed by using split_terminator("") instead, but
        // keeping that empty string at the end is actually useful so blank columns always follow a
        // math problem, including the last math problem.
        .map(|line| line.split("").skip(1))
        .collect::<Vec<_>>();

    let mut multi_line = MultiLine { iters };
    let mut grand_total = 0;

    // This time, multi_line doesn't spit out "math problem" vectors, but instead vectors containing
    // every individual character of each column. Some additional processing must be done to extract
    // the operator and operands from these columns. This processing takes advantage of the operator
    // always being contained in the first column of a math problem, and every math problem having a
    // column of blank strings after it.

    while let Some(col) = multi_line.next() {
        // For the first column, extract the operator from the final element, then use the rest of
        // the elements to extract the first operand.
        let mut col_iter = col.iter();
        let operator = *col_iter.next_back().expect("Columns should not be empty");

        let mut operands = Vec::with_capacity(4);

        operands.push(
            col_iter
                .copied()
                .collect::<String>()
                .trim()
                .parse::<u64>()
                .expect(
                    "col string should contain only numbers after extracting operator and trimming",
                ),
        );

        // For all remaining columns, parse into numbers until a fully blank column is encountered.
        loop {
            let col = multi_line
                .next()
                .expect("Puzzle input should not run out of columns while parsing a math problem");
            // The untrimmed String must be given its own let binding so the underlying String isn't
            // dropped while the trimmed version of it is borrowed.
            let operand_str_untrimmed = col.iter().copied().collect::<String>();
            let operand_str = operand_str_untrimmed.trim();

            if operand_str.is_empty() {
                // Encountering an empty string means there are no more columns to process, so
                // execute the operation.
                grand_total += if operator == "+" {
                    operands
                        .into_iter()
                        .reduce(|acc, operand| acc + operand)
                        .expect("There should be at least one operand")
                } else {
                    operands
                        .into_iter()
                        .reduce(|acc, operand| acc * operand)
                        .expect("There should be at least one operand")
                };
                break;
            }
            // If the loop wasn't broken, then this isn't an empty column, so parse the number it
            // contains.
            operands.push(operand_str.parse().expect(
                "col strings after the first column should contain only numbers after trimming if not completely blank",
            ));
        }
    }

    Solution::U64(grand_total)
}

// A custom iterator that takes in a vector of &str iterators. This outer iterator iterates over
// every source iterator simultaneously, producing a vector with the next &str from each source
// iterator until any source iterator produces None (in the context of this problem, they should all
// produce None simultaneously). This is effectively zipping the source iterators. In this case, the
// number of iterators that need to be zipped is unknown at compile time, preventing the use of
// std::iter::zip or itertools::multizip (which use fixed-size tuples) for this purpose.
struct MultiLine<'a, I: Iterator<Item = &'a str>> {
    iters: Vec<I>,
}

impl<'a, I: Iterator<Item = &'a str>> Iterator for MultiLine<'a, I> {
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iters
            .iter_mut()
            .map(Iterator::next)
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

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            ),
            Solution::U32(3_263_827)
        );
    }
}
