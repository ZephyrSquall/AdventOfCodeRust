use crate::solver::{Solution, AdventOfCode};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2024,
    day: 7,
    title: "Bridge Repair",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    // A recursive function that does a depth-first search of all possible ways the operators can be
    // allocated between operands and returns true if at least one way gives the correct result.
    fn is_equation_true(result: u64, operands: &[u64], running_total: u64, index: usize) -> bool {
        // If this is the final operand, the result from this operation must equal the input result.
        // Return true if adding or multiplying the last operand with the running total gives the
        // input result, otherwise return false.
        if index == operands.len() - 1 {
            return running_total + operands[index] == result
                || running_total * operands[index] == result;
        }

        // This is not the final operand, so call this function again twice with the next operand,
        // once with the current operand added to the running total and once with the current
        // operand multiplied with the running total.

        // All that matters is that at least one way to make the equation true exists, so recursion
        // stops as soon as one such way is found, hence these if statements immediately return true
        // if the inner function is true, but moves onto the next operation if the inner function is
        // false. Also, since every operand is 1 or greater, there is no way for addition or
        // multiplication to make the running total smaller, so if it ever becomes greater than the
        // input result, there is no need for this function to call itself any further.
        let running_total_addition = running_total + operands[index];
        if running_total_addition <= result
            && is_equation_true(result, operands, running_total_addition, index + 1)
        {
            return true;
        }

        let running_total_multiplication = running_total * operands[index];
        if running_total_multiplication <= result
            && is_equation_true(result, operands, running_total_multiplication, index + 1)
        {
            return true;
        }

        false
    }

    let mut calibration_total = 0;
    for line in input.lines() {
        let (result, operands) = get_result_and_operands(line);

        // The first step has no running total yet and starts with the first operand (at index 0).
        if is_equation_true(result, &operands, 0, 0) {
            calibration_total += result;
        }
    }

    Solution::U64(calibration_total)
}

fn solve_2(input: &str) -> Solution {
    // The only difference in part 2 is having concatenation as a third operator. Rust lacks a
    // built-in function for concatenating ints, so this must be provided myself.
    fn concatenate(a: u64, b: u64) -> u64 {
        // b.ilog10 + 1 gives the number of digits in b. Multiplying a by 10 to the power of this
        // value will shift a over however many digits is needed to fit b.
        a * 10_u64.pow(b.ilog10() + 1) + b
    }

    fn is_equation_true(result: u64, operands: &[u64], running_total: u64, index: usize) -> bool {
        if index == operands.len() - 1 {
            return running_total + operands[index] == result
                || running_total * operands[index] == result
                || concatenate(running_total, operands[index]) == result;
        }

        let running_total_addition = running_total + operands[index];
        if running_total_addition <= result
            && is_equation_true(result, operands, running_total_addition, index + 1)
        {
            return true;
        }

        let running_total_multiplication = running_total * operands[index];
        if running_total_multiplication <= result
            && is_equation_true(result, operands, running_total_multiplication, index + 1)
        {
            return true;
        }

        let running_total_concatenation = concatenate(running_total, operands[index]);
        if running_total_multiplication <= result
            && is_equation_true(result, operands, running_total_concatenation, index + 1)
        {
            return true;
        }

        false
    }

    let mut calibration_total = 0;
    for line in input.lines() {
        let (result, operands) = get_result_and_operands(line);

        if is_equation_true(result, &operands, 0, 0) {
            calibration_total += result;
        }
    }

    Solution::U64(calibration_total)
}

fn get_result_and_operands(line: &str) -> (u64, Vec<u64>) {
    let mut equation_iter = line.split(':');
    let result = equation_iter
        .next()
        .expect("Line should have a value")
        .parse()
        .expect("Line should have a number before the colon");

    let operands = equation_iter
        .next()
        .expect("Line should have values after a colon")
        .split_whitespace()
        .map(|operand_str| operand_str.parse().expect("Each value should be a number"))
        .collect();

    (result, operands)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            ),
            Solution::U16(3749)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            ),
            Solution::U16(11387)
        );
    }
}
