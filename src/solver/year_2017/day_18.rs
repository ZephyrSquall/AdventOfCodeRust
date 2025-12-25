use crate::solver::{AdventOfCode, Solution};
use rustc_hash::FxHashMap;
use std::{cell::RefCell, collections::VecDeque, rc::Rc, str::SplitWhitespace};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2017,
    day: 18,
    title: "Duet",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let instructions = get_instructions(input);
    let mut program_state = ProgramState {
        program_counter: 0,
        last_frequency: None,
        registers: FxHashMap::default(),
        is_terminating: false,
        this_message_queue: None,
        other_message_queue: None,
        sent_messages: 0,
    };

    // Repeatedly execute instructions until an instruction sets the is_terminating flag (which
    // happens then rcv is called with a non-zero value). The program counter tracks the index of
    // the next instruction to execute.
    while !program_state.is_terminating {
        instructions[program_state.program_counter].execute_set_1(&mut program_state);
    }

    if let Some(last_frequency) = program_state.last_frequency {
        Solution::I64(last_frequency)
    } else {
        panic!("No frequency was ever sounded")
    }
}

fn solve_2(input: &str) -> Solution {
    let instructions = get_instructions(input);

    // Both programs need to be able to access each others' message queues so that when they have a
    // value to send, they can push it to the other's message queue. This requires reference
    // counting with interior mutability.
    let program_0_message_queue = Rc::new(RefCell::new(VecDeque::new()));
    let program_1_message_queue = Rc::new(RefCell::new(VecDeque::new()));

    let mut program_state_0 = ProgramState {
        program_counter: 0,
        last_frequency: None,
        registers: FxHashMap::default(),
        is_terminating: false,
        this_message_queue: Some(Rc::clone(&program_0_message_queue)),
        other_message_queue: Some(Rc::clone(&program_1_message_queue)),
        sent_messages: 0,
    };
    let mut program_state_1 = ProgramState {
        program_counter: 0,
        last_frequency: None,
        registers: FxHashMap::default(),
        is_terminating: false,
        this_message_queue: Some(program_1_message_queue),
        other_message_queue: Some(program_0_message_queue),
        sent_messages: 0,
    };
    // No need to explicitly set program 0 to have register p start with value 0 since 0 is the
    // default value of a register.
    program_state_1.registers.insert('p', 1);

    // When a program attempts to receive and no value is available to be received, it remains on
    // that instruction (the program counter is not incremented) and it sets the is_terminating flag
    // to true. However, it will continue to check for a new received value every loop, and if it
    // receives one, it will set is_terminating back to false and continue executing. If both
    // programs set their is_terminating flag to true at a same time, then they have reached a
    // deadlock, so the loop exits.
    while !(program_state_0.is_terminating && program_state_1.is_terminating) {
        instructions[program_state_0.program_counter].execute_set_2(&mut program_state_0);
        instructions[program_state_1.program_counter].execute_set_2(&mut program_state_1);
    }

    Solution::U64(program_state_1.sent_messages)
}

// Representation of data necessary to run the program.
struct ProgramState {
    program_counter: usize,
    last_frequency: Option<i64>,
    registers: FxHashMap<char, i64>,
    is_terminating: bool,
    this_message_queue: Option<Rc<RefCell<VecDeque<i64>>>>,
    other_message_queue: Option<Rc<RefCell<VecDeque<i64>>>>,
    sent_messages: u64,
}
// Representation of instruction operands, which can either be a register name or an integer
// literal.
enum RegisterOrValue {
    Register(char),
    Value(i64),
}
// Representation of individual instructions.
enum Instruction {
    Snd(RegisterOrValue),
    Set(RegisterOrValue, RegisterOrValue),
    Add(RegisterOrValue, RegisterOrValue),
    Mul(RegisterOrValue, RegisterOrValue),
    Mod(RegisterOrValue, RegisterOrValue),
    Rcv(RegisterOrValue),
    Jgz(RegisterOrValue, RegisterOrValue),
}

impl RegisterOrValue {
    // Get the value from the operand, either by directly returning it if the operand is an
    // integer literal, or by fetching the value from the register if the operand is a
    // register name.
    fn get_value(&self, program_state: &ProgramState) -> i64 {
        match self {
            RegisterOrValue::Register(register) => {
                *program_state.registers.get(register).unwrap_or(&0)
            }
            RegisterOrValue::Value(value) => *value,
        }
    }
    // Get a mutable reference to the value in the operand. It's assumed this operand will only be
    // called for operands that are going to be written to (first operand of the set, add, mul, and
    // mod instructions, as well the rcv instruction for part 2), which only makes sense when the
    // operand is a register, so this function panics if the operand is a literal.
    fn get_value_mut<'a>(&self, program_state: &'a mut ProgramState) -> &'a mut i64 {
        match self {
            RegisterOrValue::Register(register) => {
                program_state.registers.entry(*register).or_insert(0)
            }
            RegisterOrValue::Value(_) => {
                panic!("Should not be getting a mutable reference to a literal operand")
            }
        }
    }
}

impl Instruction {
    // Instruction mappings for part 1.
    fn execute_set_1(&self, program_state: &mut ProgramState) {
        match self {
            Instruction::Snd(op_1) => Instruction::sound(program_state, op_1),
            Instruction::Set(op_1, op_2) => Instruction::set(program_state, op_1, op_2),
            Instruction::Add(op_1, op_2) => Instruction::add(program_state, op_1, op_2),
            Instruction::Mul(op_1, op_2) => Instruction::multiply(program_state, op_1, op_2),
            Instruction::Mod(op_1, op_2) => Instruction::modulo(program_state, op_1, op_2),
            Instruction::Rcv(op_1) => Instruction::recover(program_state, op_1),
            Instruction::Jgz(op_1, op_2) => {
                Instruction::jump_greater_zero(program_state, op_1, op_2);
            }
        }
    }
    // Instruction mappings for part 2.
    fn execute_set_2(&self, program_state: &mut ProgramState) {
        match self {
            Instruction::Snd(op_1) => Instruction::send(program_state, op_1),
            Instruction::Set(op_1, op_2) => Instruction::set(program_state, op_1, op_2),
            Instruction::Add(op_1, op_2) => Instruction::add(program_state, op_1, op_2),
            Instruction::Mul(op_1, op_2) => Instruction::multiply(program_state, op_1, op_2),
            Instruction::Mod(op_1, op_2) => Instruction::modulo(program_state, op_1, op_2),
            Instruction::Rcv(op_1) => Instruction::receive(program_state, op_1),
            Instruction::Jgz(op_1, op_2) => {
                Instruction::jump_greater_zero(program_state, op_1, op_2);
            }
        }
    }

    // Shared instructions
    fn set(program_state: &mut ProgramState, op_1: &RegisterOrValue, op_2: &RegisterOrValue) {
        let op_2 = op_2.get_value(program_state);
        let op_1 = op_1.get_value_mut(program_state);
        *op_1 = op_2;
        program_state.program_counter += 1;
    }
    fn add(program_state: &mut ProgramState, op_1: &RegisterOrValue, op_2: &RegisterOrValue) {
        let op_2 = op_2.get_value(program_state);
        let op_1 = op_1.get_value_mut(program_state);
        *op_1 += op_2;
        program_state.program_counter += 1;
    }
    fn multiply(program_state: &mut ProgramState, op_1: &RegisterOrValue, op_2: &RegisterOrValue) {
        let op_2 = op_2.get_value(program_state);
        let op_1 = op_1.get_value_mut(program_state);
        *op_1 *= op_2;
        program_state.program_counter += 1;
    }
    fn modulo(program_state: &mut ProgramState, op_1: &RegisterOrValue, op_2: &RegisterOrValue) {
        let op_2 = op_2.get_value(program_state);
        let op_1 = op_1.get_value_mut(program_state);
        *op_1 %= op_2;
        program_state.program_counter += 1;
    }
    fn jump_greater_zero(
        program_state: &mut ProgramState,
        op_1: &RegisterOrValue,
        op_2: &RegisterOrValue,
    ) {
        let op_1 = op_1.get_value(program_state);
        if op_1 > 0 {
            let op_2 = op_2.get_value(program_state);
            if op_2.is_negative() {
                program_state.program_counter -= usize::try_from(op_2.unsigned_abs())
                    .expect("Should be able to convert to usize losslessly");
            } else {
                program_state.program_counter +=
                    usize::try_from(op_2).expect("Should be able to convert to usize losslessly");
            }
        } else {
            program_state.program_counter += 1;
        }
    }

    // Part 1 exclusive instructions
    fn sound(program_state: &mut ProgramState, op_1: &RegisterOrValue) {
        let op_1 = op_1.get_value(program_state);
        program_state.last_frequency = Some(op_1);
        program_state.program_counter += 1;
    }
    fn recover(program_state: &mut ProgramState, op_1: &RegisterOrValue) {
        let op_1 = op_1.get_value(program_state);
        if op_1 != 0 {
            program_state.is_terminating = true;
        } else {
            program_state.program_counter += 1;
        }
    }

    // Part 2 exclusive instructions
    fn send(program_state: &mut ProgramState, op_1: &RegisterOrValue) {
        let op_1 = op_1.get_value(program_state);
        // Dereference the Rc and RefCell to access the message queue within and push to it.
        program_state
            .other_message_queue
            .as_ref()
            .expect("Should have a message queue")
            .as_ref()
            .borrow_mut()
            .push_back(op_1);
        program_state.sent_messages += 1;
        program_state.program_counter += 1;
    }
    fn receive(program_state: &mut ProgramState, op_1: &RegisterOrValue) {
        // Dereference the Rc and RefCell to access the message queue within and retrieve the front
        // value from it.
        let value = program_state
            .this_message_queue
            .as_ref()
            .expect("Should have a message queue")
            .as_ref()
            .borrow_mut()
            .pop_front();
        match value {
            Some(value) => {
                let op_1 = op_1.get_value_mut(program_state);
                *op_1 = value;
                // It's possible this code was blocking for some time and set the is_terminating
                // flag to true, so explicitly set it back to false now that a value has been
                // received and execution can continue.
                program_state.is_terminating = false;
                program_state.program_counter += 1;
            }
            None => {
                // If popping returned none, the other program has not yet sent a value, so block
                // until one is available. This is achieved by not incrementing the program counter
                // so this instruction will continue to be executed until a value is available. The
                // is_terminating flag is also set so the outer code can detect when the deadlock
                // occurs.
                program_state.is_terminating = true;
            }
        }
    }
}

fn get_instructions(input: &str) -> Vec<Instruction> {
    // Helper function to extract an operand.
    fn get_operand(iter: &mut SplitWhitespace) -> RegisterOrValue {
        let op = iter.next().expect("Line should have third value");

        if let Ok(value) = op.parse() {
            RegisterOrValue::Value(value)
        } else {
            let char = op
                .chars()
                .next()
                .expect("Operand should be a number or single character");
            RegisterOrValue::Register(char)
        }
    }

    let mut instructions = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let instruction = iter.next().expect("Line should have first value");
        let instruction = match instruction {
            "snd" => {
                let op_1 = get_operand(&mut iter);
                Instruction::Snd(op_1)
            }
            "set" => {
                let op_1 = get_operand(&mut iter);
                let op_2 = get_operand(&mut iter);
                Instruction::Set(op_1, op_2)
            }
            "add" => {
                let op_1 = get_operand(&mut iter);
                let op_2 = get_operand(&mut iter);
                Instruction::Add(op_1, op_2)
            }
            "mul" => {
                let op_1 = get_operand(&mut iter);
                let op_2 = get_operand(&mut iter);
                Instruction::Mul(op_1, op_2)
            }
            "mod" => {
                let op_1 = get_operand(&mut iter);
                let op_2 = get_operand(&mut iter);
                Instruction::Mod(op_1, op_2)
            }
            "rcv" => {
                let op_1 = get_operand(&mut iter);
                Instruction::Rcv(op_1)
            }
            "jgz" => {
                let op_1 = get_operand(&mut iter);
                let op_2 = get_operand(&mut iter);
                Instruction::Jgz(op_1, op_2)
            }
            _ => panic!("Invalid instruction name"),
        };
        instructions.push(instruction);
    }

    instructions
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2"
            ),
            Solution::U8(4)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d
"
            ),
            Solution::U8(3)
        );
    }
}
