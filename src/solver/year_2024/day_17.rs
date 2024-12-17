use crate::solver::{Solution, Solver};
use itertools::Itertools;

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 17,
    title: "Chronospatial Computer",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    struct Registers {
        a: u32,
        b: u32,
        c: u32,
    }

    // A program value represents one of the three-bit numbers in a program. It contains several
    // methods for either converting a u32 into a new program value, or reading the program value as
    // an operand.
    enum ProgramValue {
        Zero,
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
    }
    impl ProgramValue {
        fn new(value: &str) -> ProgramValue {
            match value {
                "0" => ProgramValue::Zero,
                "1" => ProgramValue::One,
                "2" => ProgramValue::Two,
                "3" => ProgramValue::Three,
                "4" => ProgramValue::Four,
                "5" => ProgramValue::Five,
                "6" => ProgramValue::Six,
                "7" => ProgramValue::Seven,
                _ => panic!("Value was not a number from 0 to 7"),
            }
        }
        fn read_literal_operand(&self) -> u32 {
            match self {
                ProgramValue::Zero => 0,
                ProgramValue::One => 1,
                ProgramValue::Two => 2,
                ProgramValue::Three => 3,
                ProgramValue::Four => 4,
                ProgramValue::Five => 5,
                ProgramValue::Six => 6,
                ProgramValue::Seven => 7,
            }
        }
        fn read_combo_operand(&self, registers: &Registers) -> u32 {
            match self {
                ProgramValue::Zero => 0,
                ProgramValue::One => 1,
                ProgramValue::Two => 2,
                ProgramValue::Three => 3,
                ProgramValue::Four => registers.a,
                ProgramValue::Five => registers.b,
                ProgramValue::Six => registers.c,
                ProgramValue::Seven => {
                    panic!("7 does not appear as a combo operand in valid programs")
                }
            }
        }
    }

    // Given the opcode, operand, and current register values, execute the instruction corresponding
    // to the opcode. This may mutate the instruction pointer and the outputs, so also provide those
    // as mutable arguments.
    fn execute(
        opcode: &ProgramValue,
        operand: &ProgramValue,
        registers: &mut Registers,
        instruction_pointer: &mut usize,
        outputs: &mut Vec<u32>,
    ) {
        match opcode {
            ProgramValue::Zero => {
                // adv
                let operand_value = operand.read_combo_operand(registers);
                registers.a /= 2_u32.pow(operand_value);
                *instruction_pointer += 2;
            }
            ProgramValue::One => {
                // bxl
                let operand_value = operand.read_literal_operand();
                registers.b ^= operand_value;
                *instruction_pointer += 2;
            }
            ProgramValue::Two => {
                // bst
                let operand_value = operand.read_combo_operand(registers);
                registers.b = operand_value % 8;
                *instruction_pointer += 2;
            }
            ProgramValue::Three => {
                // jnz
                if registers.a == 0 {
                    *instruction_pointer += 2;
                } else {
                    let operand_value = operand.read_literal_operand();
                    *instruction_pointer = operand_value as usize;
                }
            }
            ProgramValue::Four => {
                // bxc
                registers.b ^= registers.c;
                *instruction_pointer += 2;
            }
            ProgramValue::Five => {
                // out
                let operand_value = operand.read_combo_operand(registers);
                outputs.push(operand_value % 8);
                *instruction_pointer += 2;
            }
            ProgramValue::Six => {
                // bdv
                let operand_value = operand.read_combo_operand(registers);
                registers.b = registers.a / 2_u32.pow(operand_value);
                *instruction_pointer += 2;
            }
            ProgramValue::Seven => {
                // cdv
                let operand_value = operand.read_combo_operand(registers);
                registers.c = registers.a / 2_u32.pow(operand_value);
                *instruction_pointer += 2;
            }
        }
    }

    // Get the initial register values
    let mut line_iter = input.lines();
    let register_a = line_iter
        .next()
        .expect("Input should have first line")
        .trim_start_matches("Register A: ")
        .parse()
        .expect("First line should be a number after trimming");
    let register_b = line_iter
        .next()
        .expect("Input should have second line")
        .trim_start_matches("Register B: ")
        .parse()
        .expect("Second line should be a number after trimming");
    let register_c = line_iter
        .next()
        .expect("Input should have third line")
        .trim_start_matches("Register C: ")
        .parse()
        .expect("Third line should be a number after trimming");
    let mut registers = Registers {
        a: register_a,
        b: register_b,
        c: register_c,
    };

    // Get the program (a vector of ProgramValues)
    let program = line_iter
        .nth(1)
        .expect("Input should have fourth and fifth lines")
        .trim_start_matches("Program: ")
        .split(',')
        .map(ProgramValue::new)
        .collect::<Vec<_>>();

    // Initialize the instruction pointer and output.
    let mut instruction_pointer = 0;
    let mut outputs = Vec::new();

    // Execute instructions until the program halts (which occurs when it attempts to read a value
    // outside of the program, in which case a None would be returned from one of the get methods)
    while let Some((opcode, operand)) = program
        .get(instruction_pointer)
        .zip(program.get(instruction_pointer + 1))
    {
        execute(
            opcode,
            operand,
            &mut registers,
            &mut instruction_pointer,
            &mut outputs,
        );
    }

    // Convert the output integers into a string of digits separated by commas.
    let output_string = outputs.into_iter().join(",");
    Solution::String(output_string)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
            ),
            Solution::Str("4,6,3,5,6,3,5,2,1,0")
        );
    }
}
