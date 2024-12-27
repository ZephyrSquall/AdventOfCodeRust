use crate::solver::{Solution, AdventOfCode};
use rustc_hash::FxHashMap;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2017,
    day: 8,
    title: "I Heard You Like Registers",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let mut registers = FxHashMap::default();

    for line in input.lines() {
        let instruction = get_instruction(line);

        let condition_register_ref = registers.entry(instruction.condition_register).or_insert(0);
        if test_condition(
            *condition_register_ref,
            instruction.condition_amount,
            &instruction.condition,
        ) {
            let operation_register_ref =
                registers.entry(instruction.operation_register).or_insert(0);
            match instruction.operation {
                Operation::Increase => *operation_register_ref += instruction.operation_amount,
                Operation::Decrease => *operation_register_ref -= instruction.operation_amount,
            }
        }
    }

    let largest_register_ref = registers
        .values()
        .max()
        .expect("Error finding maximum value");
    Solution::I32(*largest_register_ref)
}

fn solve_2(input: &str) -> Solution {
    let mut registers = FxHashMap::default();
    let mut largest_value = i32::MIN;

    for line in input.lines() {
        let instruction = get_instruction(line);

        let condition_register_ref = registers.entry(instruction.condition_register).or_insert(0);
        if test_condition(
            *condition_register_ref,
            instruction.condition_amount,
            &instruction.condition,
        ) {
            let operation_register_ref =
                registers.entry(instruction.operation_register).or_insert(0);
            match instruction.operation {
                Operation::Increase => *operation_register_ref += instruction.operation_amount,
                Operation::Decrease => *operation_register_ref -= instruction.operation_amount,
            }

            if *operation_register_ref > largest_value {
                largest_value = *operation_register_ref;
            }
        }
    }

    Solution::I32(largest_value)
}

enum Operation {
    Increase,
    Decrease,
}

enum Condition {
    EqualTo,
    NotEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
}

struct Instruction<'a> {
    operation_register: &'a str,
    operation: Operation,
    operation_amount: i32,
    condition_register: &'a str,
    condition: Condition,
    condition_amount: i32,
}

fn get_instruction(line: &str) -> Instruction {
    let mut iter = line.split_whitespace();
    let operation_register = iter.next().expect("Missing value");
    let operation = match iter.next() {
        Some("inc") => Operation::Increase,
        Some("dec") => Operation::Decrease,
        _ => panic!(),
    };
    let operation_amount: i32 = iter
        .next()
        .expect("Missing value")
        .parse()
        .expect("Error parsing number");
    // Ignore the "if"
    iter.next();
    let condition_register = iter.next().expect("Missing value");
    let condition = match iter.next() {
        Some("==") => Condition::EqualTo,
        Some("!=") => Condition::NotEqualTo,
        Some(">") => Condition::GreaterThan,
        Some(">=") => Condition::GreaterThanOrEqualTo,
        Some("<") => Condition::LessThan,
        Some("<=") => Condition::LessThanOrEqualTo,
        _ => panic!(),
    };
    let condition_amount: i32 = iter
        .next()
        .expect("Missing value")
        .parse()
        .expect("Error parsing number");

    Instruction {
        operation_register,
        operation,
        operation_amount,
        condition_register,
        condition,
        condition_amount,
    }
}

fn test_condition(a: i32, b: i32, condition: &Condition) -> bool {
    match condition {
        Condition::EqualTo => a == b,
        Condition::NotEqualTo => a != b,
        Condition::GreaterThan => a > b,
        Condition::GreaterThanOrEqualTo => a >= b,
        Condition::LessThan => a < b,
        Condition::LessThanOrEqualTo => a <= b,
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
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"
            ),
            Solution::U8(1)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"
            ),
            Solution::U8(10)
        );
    }
}
