use crate::solver::{Solution, Solver};
use std::str::SplitWhitespace;

pub const SOLVER: Solver = Solver {
    year: 2017,
    day: 23,
    title: "Coprocessor Conflagration",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let instructions = get_instructions(input);
    let mut program_state = ProgramState {
        program_counter: 0,
        registers: [0; 8],
        mul_invocations: 0,
    };

    // The puzzle description does not state under what conditions the code in the puzzle input
    // terminates. If we assume that the puzzle has a finite answer, then at some point, the mul
    // instruction must stop getting invoked. I can think of two possibilities that can cause this
    // without an explicit termination condition: The program eventually falls into an infinite loop
    // that contains no mul instructions, or the program eventually jumps to an instruction that is
    // outside of the code. Through testing my own puzzle input, I have verified that the latter is
    // true (my code eventually jumps to an instruction outside of the code), so I will assume this
    // is the intended termination condition for all puzzle inputs.
    while let Some(instruction) = instructions.get(program_state.program_counter) {
        instruction.execute(&mut program_state);
    }

    Solution::U32(program_state.mul_invocations)
}

fn solve_2(input: &str) -> Solution {
    // This puzzle required making some extreme assumptions about the puzzle input. By stepping
    // through my puzzle input step-by-step and mapping out the logic, I determined that the initial
    // values in the b and c registers define an inclusive lower and upper bound for a range of
    // values, which are offset from each other by a hardcoded value in the "sub b -<SOME VALUE>"
    // instruction on the second-last line (e.g. if b is 4, c is 10, and the hardcoded value is 2,
    // the values that the program would iterate over are be 4, 6, 8, 10). For each of these values,
    // it sets register f to 1, then it creates a sub-loop using register d, which itself has its
    // own sub-loop using register e, both of which count from 2 to the current value. In every
    // iteration of the innermost loop, it checks if d*e - b == 0, and if this is true, it sets f to
    // 0. Once the outer sub-loop completes, if f was set to 0, register h is incremented by 1.
    //
    // The condition d*e = b == 0 is equivalent to d*e == b. If this is never true, then register f
    // remains set to 1 and h isn't incremented; if it is true at least one time, then register f is
    // set to 0 and h gets incremented. As such, for each value, the program essentially asks "Does
    // there exist any two integers which are both greater than or equal to 2, such that their
    // product equals the given value?" This is equivalent to asking if the given value is a
    // composite number (a positive integer other than 1 which is not a prime number). As such, what
    // the program ultimately does is count every composite number within the set of numbers defined
    // by the bounds of registers b and c and the hardcoded offset, and stores the count in register
    // h.
    //
    // I assume this is the intended trick of the puzzle, and that every puzzle input is set up to
    // follow the rules described above. As such, my solution searches the program in the puzzle
    // input for every line that modifies registers b or c, executes all of them except for the last
    // one, assumes the values for b and c I end up with are the initial values for them, and
    // assumes the literal argument to the last instruction instead defines the offset. I ignore all
    // other lines in the puzzle input, assuming that the rest of it is just implementation details
    // for counting composite numbers. I use the extracted b, c, and offset values to set up the
    // full list of values, then use my own implementation for counting composite numbers. If any of
    // the assumptions I have made for this solution are incorrect, then my solution might not work
    // for all possible puzzle inputs.

    fn is_prime(x: u32) -> bool {
        // Technically, an is_prime() function would need to check if the input is 1 or lower and
        // immediately return false if so, but it's known this function will never be ran for a
        // value smaller than 2 within this solver, so this check is omitted.

        // When checking for prime numbers, only integers up to the square root of the number need
        // to be checked (any factor greater than the square root will need to be multiplied by a
        // factor smaller than the square root to equal the given number, so if the number isn't
        // prime, it will always have at least one factor (other than 1) that is smaller than its
        // square root).

        // Intentionally truncating the float (this is accounted for by using an inclusive range
        // "..=" which includes the end instead of a normal range ".." which excludes the end).
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        for test in 2..=(f64::from(x)).sqrt() as u32 {
            if x % test == 0 {
                return false;
            }
        }
        true
    }

    // Predicate that says whether the given instruction will modify the value in the b or c
    // registers.
    fn is_writing_to_b_or_c(instruction: &Instruction) -> bool {
        match instruction {
            Instruction::Set(op_1, _) | Instruction::Sub(op_1, _) | Instruction::Mul(op_1, _) => {
                if let RegisterOrValue::Register(index) = op_1 {
                    // Registers b and c have indexes 1 and 2 respectively.
                    return *index == 1 || *index == 2;
                }
                false
            }
            // The jnz function doesn't modify any registers so always return false (in principle
            // they might affect b and c through looping around another instruction that affects it,
            // however since only the initial values of b and c are needed, it's assumed that once
            // jumping is done to begin looping, the initial values of b and c have been finalized
            // and this jnz functions can still be safely ignored).
            Instruction::Jnz(_, _) => false,
        }
    }

    let mut instructions = get_instructions(input);
    // Remove all instructions that don't modify b or c.
    instructions.retain(is_writing_to_b_or_c);

    let mut program_state = ProgramState {
        program_counter: 0,
        registers: [1, 0, 0, 0, 0, 0, 0, 0],
        mul_invocations: 0,
    };

    let mut instruction_iter = instructions.iter();

    // First take the last value off the list of instructions and use it to find the offset.
    let last_instruction = instruction_iter
        .next_back()
        .expect("Instructions should have last element");
    let offset = if let Instruction::Sub(_, op_2) = last_instruction {
        if let RegisterOrValue::Value(value) = op_2 {
            // value is expected to be negative as it's used in a sub instruction with the intention
            // of adding an amount to register b, so the absolute value of it must be taken.
            value.unsigned_abs()
        } else {
            panic!("Offset should be an integer literal")
        }
    } else {
        panic!("Last instruction should be a sub instruction to add the offset")
    };

    // Execute all instructions except the last one (which is no longer in instruction_iter).
    for instruction in instruction_iter {
        instruction.execute(&mut program_state);
    }

    // It's expected that these values are positive. unsigned_abs() helps convert to u32.
    // Register b:
    let mut range_start = program_state.registers[1].unsigned_abs();
    // Register c:
    let range_end = program_state.registers[2].unsigned_abs();

    let mut values_in_range = Vec::with_capacity(1001);

    while range_start <= range_end {
        values_in_range.push(range_start);
        range_start += offset;
    }

    // Count composite numbers.
    let mut composites = 0;
    for value in values_in_range {
        if !is_prime(value) {
            composites += 1;
        }
    }

    return Solution::U32(composites);
}

struct ProgramState {
    program_counter: usize,
    registers: [i32; 8],
    mul_invocations: u32,
}
// Note that unlike in day 18, it is specified that only the first 8 letters ('a' to 'h') are used.
// This is taken advantage of so that each letter is an index into an array of 8 ints, rather than a
// HashMap. Hence the Register enum uses a usize instead of a char here.
enum RegisterOrValue {
    Register(usize),
    Value(i32),
}
// Representation of individual instructions.
enum Instruction {
    Set(RegisterOrValue, RegisterOrValue),
    Sub(RegisterOrValue, RegisterOrValue),
    Mul(RegisterOrValue, RegisterOrValue),
    Jnz(RegisterOrValue, RegisterOrValue),
}

impl RegisterOrValue {
    // Get the value from the operand, either by directly returning it if the operand is an integer
    // literal, or by fetching the value from the register if the operand is a register name.
    fn get_value(&self, program_state: &ProgramState) -> i32 {
        match self {
            RegisterOrValue::Register(index) => program_state.registers[*index],
            RegisterOrValue::Value(value) => *value,
        }
    }
    // Get a mutable reference to the value in the operand. It's assumed this operand will only be
    // called for operands that are going to be written to (first operand of the set, sub, and mul
    // instructions), which only makes sense when the operand is a register, so this function panics
    // if the operand is a literal.
    fn get_value_mut<'a>(&self, program_state: &'a mut ProgramState) -> &'a mut i32 {
        match self {
            RegisterOrValue::Register(index) => &mut program_state.registers[*index],
            RegisterOrValue::Value(_) => {
                panic!("Should not be getting a mutable reference to a literal operand")
            }
        }
    }
}

impl Instruction {
    // Instruction mappings for part 1.
    fn execute(&self, program_state: &mut ProgramState) {
        match self {
            Instruction::Set(op_1, op_2) => {
                let op_2 = op_2.get_value(program_state);
                let op_1 = op_1.get_value_mut(program_state);
                *op_1 = op_2;
                program_state.program_counter += 1;
            }
            Instruction::Sub(op_1, op_2) => {
                let op_2 = op_2.get_value(program_state);
                let op_1 = op_1.get_value_mut(program_state);
                *op_1 -= op_2;
                program_state.program_counter += 1;
            }
            Instruction::Mul(op_1, op_2) => {
                let op_2 = op_2.get_value(program_state);
                let op_1 = op_1.get_value_mut(program_state);
                *op_1 *= op_2;
                program_state.program_counter += 1;
                program_state.mul_invocations += 1;
            }
            Instruction::Jnz(op_1, op_2) => {
                let op_1 = op_1.get_value(program_state);
                if op_1 != 0 {
                    let op_2 = op_2.get_value(program_state);
                    if op_2.is_negative() {
                        program_state.program_counter -= usize::try_from(op_2.unsigned_abs())
                            .expect("Should be able to convert to usize losslessly");
                    } else {
                        program_state.program_counter += usize::try_from(op_2)
                            .expect("Should be able to convert to usize losslessly");
                    }
                } else {
                    program_state.program_counter += 1;
                }
            }
        }
    }
}

fn get_instructions(input: &str) -> Vec<Instruction> {
    // Helper function to extract an operand.
    fn get_operand(iter: &mut SplitWhitespace) -> RegisterOrValue {
        let op = iter.next().expect("Line should have third value");
        let register_or_value = if let Ok(value) = op.parse() {
            RegisterOrValue::Value(value)
        } else {
            let mut index = op
                .chars()
                .next()
                .expect("Operand should be a number or single character")
                as usize;
            // ASCII value of 'a' is 97, so subtracting 97 gives an index from 0 to 7 (7
            // corresponding to 'h' with ASCII value 104).
            index -= 97;

            RegisterOrValue::Register(index)
        };

        register_or_value
    }

    let mut instructions = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let instruction = iter.next().expect("Line should have first value");
        let instruction = match instruction {
            "set" => {
                let op_1 = get_operand(&mut iter);
                let op_2 = get_operand(&mut iter);
                Instruction::Set(op_1, op_2)
            }
            "sub" => {
                let op_1 = get_operand(&mut iter);
                let op_2 = get_operand(&mut iter);
                Instruction::Sub(op_1, op_2)
            }
            "mul" => {
                let op_1 = get_operand(&mut iter);
                let op_2 = get_operand(&mut iter);
                Instruction::Mul(op_1, op_2)
            }
            "jnz" => {
                let op_1 = get_operand(&mut iter);
                let op_2 = get_operand(&mut iter);
                Instruction::Jnz(op_1, op_2)
            }
            _ => panic!("Invalid instruction name"),
        };
        instructions.push(instruction);
    }

    instructions
}

// The puzzle description provides no examples for this puzzle.
