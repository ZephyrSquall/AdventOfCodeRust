use crate::solver::{Solution, AdventOfCode};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2015,
    day: 23,
    title: "Opening the Turing Lock",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    solve(input, [0, 0])
}

fn solve_2(input: &str) -> Solution {
    solve(input, [1, 0])
}

// registers[0] is a, registers[1] is b.
fn solve(input: &str, mut registers: [u32; 2]) -> Solution {
    // Get all instructions.
    let mut instructions = Vec::new();
    for line in input.lines() {
        let mut instruction_iter = line.split(' ');
        let instruction_str = instruction_iter
            .next()
            .expect("Line should have instruction code");

        match instruction_str {
            "hlf" => instructions.push(Instruction::Hlf(get_register_arg(instruction_iter))),
            "tpl" => instructions.push(Instruction::Tpl(get_register_arg(instruction_iter))),
            "inc" => instructions.push(Instruction::Inc(get_register_arg(instruction_iter))),
            "jmp" => instructions.push(Instruction::Jmp(get_offset_arg(instruction_iter))),
            "jie" => {
                let (register, offset) = get_register_and_offset_args(instruction_iter);
                instructions.push(Instruction::Jie(register, offset));
            }
            "jio" => {
                let (register, offset) = get_register_and_offset_args(instruction_iter);
                instructions.push(Instruction::Jio(register, offset));
            }
            _ => panic!("Unsupported instruction"),
        }
    }

    let mut program_counter = 0;

    // Execute instructions until the program counter reaches an invalid index.
    while let Some(instruction) = instructions.get(program_counter) {
        instruction.execute(&mut program_counter, &mut registers);
    }

    Solution::U32(registers[1])
}

fn get_register_arg<'a>(mut instruction_iter: impl Iterator<Item = &'a str>) -> usize {
    let register_str = instruction_iter
        .next()
        .expect("Instruction should have an argument");
    match register_str {
        "a" => 0,
        "b" => 1,
        _ => panic!("Unsupported register"),
    }
}

fn get_offset_arg<'a>(mut instruction_iter: impl Iterator<Item = &'a str>) -> isize {
    instruction_iter
        .next()
        .expect("Instruction should have an argument")
        .parse()
        .expect("Offset should be a number")
}

fn get_register_and_offset_args<'a>(
    mut instruction_iter: impl Iterator<Item = &'a str>,
) -> (usize, isize) {
    let register_str = instruction_iter
        .next()
        .expect("Instruction should have an argument");
    let register = match register_str.trim_end_matches(',') {
        "a" => 0,
        "b" => 1,
        _ => panic!("Unsupported register"),
    };
    let offset = instruction_iter
        .next()
        .expect("Instruction should have an argument")
        .parse()
        .expect("Offset should be a number");
    (register, offset)
}

// usize refers to register indexes, isize refers to jump instruction offsets.
enum Instruction {
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(isize),
    Jie(usize, isize),
    Jio(usize, isize),
}
impl Instruction {
    fn execute(&self, program_counter: &mut usize, registers: &mut [u32; 2]) {
        match self {
            Instruction::Hlf(register_index) => {
                registers[*register_index] /= 2;
                *program_counter += 1;
            }
            Instruction::Tpl(register_index) => {
                registers[*register_index] *= 3;
                *program_counter += 1;
            }
            Instruction::Inc(register_index) => {
                registers[*register_index] += 1;
                *program_counter += 1;
            }
            Instruction::Jmp(offset) => {
                // If the program counter would go below 0, it would wrap around to a ludicrously
                // high index that should definitely be out-of-bounds (the length of the program in
                // my puzzle input is orders of magnitude less than usize::MAX).
                *program_counter = program_counter.wrapping_add_signed(*offset);
            }
            Instruction::Jie(register_index, offset) => {
                if registers[*register_index] % 2 == 0 {
                    *program_counter = program_counter.wrapping_add_signed(*offset);
                } else {
                    *program_counter += 1;
                }
            }
            Instruction::Jio(register_index, offset) => {
                if registers[*register_index] == 1 {
                    *program_counter = program_counter.wrapping_add_signed(*offset);
                } else {
                    *program_counter += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // This test is modified to change register b instead of register a, as the real puzzle cares
    // about register b's final value.
    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
inc b
jio b, +2
tpl b
inc b"
            ),
            Solution::U8(2)
        );
    }
}
