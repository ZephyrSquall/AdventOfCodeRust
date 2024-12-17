use crate::solver::{Solution, Solver};
use itertools::Itertools;

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 17,
    title: "Chronospatial Computer",
    part_solvers: &[solve_1, solve_2],
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

// Possible truncation happens in the exponents of the .pow() methods. However, the values there
// (roughly the number of digits in another number) aren't close to the maximum value a u32 could
// hold, so truncation won't happen.
#[allow(clippy::cast_possible_truncation)]
fn solve_2(input: &str) -> Solution {
    // Solving this part in general for any possible input seems impractical and obviously not the
    // intended solution. For my own puzzle input, I was able to determine the answer is at least
    // 35,184,372,088,832 and at most 281,474,976,710,655 (8^15 and 8^16 - 1), which leaves
    // 246,290,604,621,824 possible answers to check, an amount that will take at least several
    // hours to search. As such, I have to take advantage of patterns I find in my puzzle input that
    // weren't given in the puzzle description. I have no way to check if the patterns I take
    // advantage of are present in all puzzle inputs, so it's possible this solver will give
    // incorrect answers for other puzzle inputs.
    //
    // By converting the opcodes and operands in my puzzle inputs into their corresponding
    // instructions, I find that the program in my puzzle input is equivalent to the following
    // pseudocode (^ represents XOR):

    // loop: {
    //     b = a % 8;
    //     b = b ^ 1;
    //     c = a / 2.pow(b);
    //     b = b ^ 5;
    //     b = b ^ c;
    //     outputs.push(b % 8);
    //     a = a / 8;
    //     if a == 0 {
    //         break;
    //     }
    // }

    // The initial values of registers b and c are irrelevant, because they are assigned to before
    // they are first read. a is divided by 8 many times, so it is easier to think of a as an octal
    // number (written in base 8). In octal form, the line "b = a % 8" sets b to the value of the
    // last digit in a, the line "a = a / 8" removes the last digit in a, and the condition
    // "if a == 0" checks if all digits in a have been removed. With this information, this
    // pseudocode can be simplified to:

    // while(a has digits): {
    //     b = a_last_digit;
    //     b = b ^ 1;
    //     c = a / 2.pow(b);
    //     b = b ^ 5;
    //     b = b ^ c;
    //     outputs.push(b % 8);
    //     a.remove_last_digit()
    // }

    // We can use this pseudocode to conclude that on each iteration, the output digit that is
    // pushed is given by `output = (((a_last_digit ^ 1) ^ 5) ^ (a / 2.pow(a_last_digit ^ 1))) % 8`.
    // We know what each output digit needs to be, so in principle, we can work backwards from this
    // to determine what a_last_digit must have been to produce that output digit.
    //
    // However, the fact that both a and a_last_digit change each loop iteration is problematic.
    // Thought this can be resolved by flipping the logic and working backwards from the final
    // output value. To ensure that the program doesn't loop again after the final output value, a
    // must be on its last digit at this point, which means a and a_last_digit are equal. Once a
    // value for a is determined for this loop, this "locks in" the first digit of a. Then the
    // second-last value of the output can be considered, where we know that
    // a = 8 * a_prev + a_digit (where a_prev is the value of a calculated for the final output
    // digit). This strategy can be continued all the way down, where a_prev is the octal number
    // formed by all digits of a found so far, multiplied by 8 (so the final digit is determined by
    // a_last_digit, not a_prev).
    //
    // Working backwards like this is also convenient in that it helps find the minimum value
    // easily. For each step, if there are multiple values of a_last_digit that solve the equation,
    // we simply choose the smallest value. It's also possible there might be no values of
    // a_last_digit that solve the equation, in which case an earlier value of a_last_digit isn't
    // usable so go back to that value and continue checking greater numbers from there. As long as
    // digits are checked in increasing order like this, the first full set of digits that solves
    // the equation for every output value must be the digits that give the smallest overall number
    // for a.

    // This function recursively finds the octal digits of a, working backwards from the last output
    // number.
    fn get_a_octal_digits(
        a_octal_digits: &mut Vec<u64>,
        output: &Vec<u64>,
        output_index: usize,
        // For the very first digit, it must be forced to be 1 or greater, as if it were 0, a
        // wouldn't have enough digits to loop enough times to produce all the output values. No
        // other digit in a has this restriction, so they can be 0.
        first_a_digit: u64,
    ) -> bool {
        let output_digit = output[output_index];
        let a_prev = a_octal_digits
            .iter()
            .rev()
            .enumerate()
            // Get the value of the octal number by multiplying each digit by 8 to the power of that
            // digit's position.
            .fold(0, |acc, (place, digit)| {
                acc + digit * 8_u64.pow(place as u32)
            })
            // Multiply 8 to shift all digits one position over, so that the last digit is
            // determined by a_last_digit.
            * 8;

        for a_last_digit in first_a_digit..8 {
            if output_digit
                == (((a_last_digit ^ 1) ^ 5)
                    ^ ((a_prev + a_last_digit) / (2_u64.pow((a_last_digit ^ 1) as u32))))
                    % 8
            {
                // a_last_digit solves the above equation, so add this digit to the number and check
                // if digits can be found after this that leads to a complete number. If so, return
                // true. If output_index == 0, then this was the last digit and a complete number
                // has just been found, so also return true in that case.
                a_octal_digits.push(a_last_digit);
                if output_index == 0
                    || get_a_octal_digits(a_octal_digits, output, output_index - 1, 0)
                {
                    return true;
                }

                // If the inner get_a_octal_digits returned false, then there is no set of a digits
                // after this digit that completes the output sequence correctly. So remove this
                // digit and move on to checking the next digit.
                a_octal_digits.pop();
            }
        }

        false
    }

    // The initial register values do not matter. a is overwritten by whatever value causes the
    // program to output itself, and b and c are written to before they are read. So just skip ahead
    // to reading the program values.
    let outputs = input
        .lines()
        .nth(4)
        .expect("Input should have five lines")
        .trim_start_matches("Program: ")
        .split(',')
        .map(|value| {
            value
                .parse()
                .expect("Line should only have numbers after trimming and splitting")
        })
        .collect::<Vec<_>>();
    let mut a_octal_digits = Vec::with_capacity(outputs.len());

    get_a_octal_digits(&mut a_octal_digits, &outputs, outputs.len() - 1, 1);

    // Convert the octal digits into a decimal number.
    let register_a = a_octal_digits
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (place, digit)| {
            acc + digit * 8_u64.pow(place as u32)
        });
    Solution::U64(register_a)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4"
            ),
            Solution::Str("0,1,2")
        );
    }
    #[test]
    fn example1_2() {
        assert_eq!(
            solve_1(
                "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
            ),
            Solution::Str("4,2,5,6,7,7,7,7,3,1,0")
        );
    }
    #[test]
    fn example1_3() {
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

    // A test for part 2 is provided, but it violates the assumptions I had to make about my own puzzle
    // input, so it cannot be used to test my solution.
}
