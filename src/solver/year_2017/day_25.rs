use crate::solver::{Solution, AdventOfCode};
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2017,
    day: 25,
    title: "The Halting Problem",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let blueprint = get_blueprint(input);
    let mut tape = Tape {
        slots: vec![false].into(),
        position: 0,
    };
    let mut state_name = blueprint.first_state;

    for _ in 0..blueprint.iterations {
        let state = blueprint
            .states
            .get(&state_name)
            .expect("All states should be present in hash map");

        // Read the tape's value under its cursor, execute the corresponding instruction, and
        // overwrite state_name with the return value from Instruction::execute() which is the next
        // state's name.
        state_name = if tape.read() {
            state.if_1.execute(&mut tape)
        } else {
            state.if_0.execute(&mut tape)
        };
    }

    Solution::U32(tape.diagnostic_checksum())
}

struct Tape {
    slots: VecDeque<bool>,
    position: usize,
}
impl Tape {
    fn read(&self) -> bool {
        self.slots[self.position]
    }
    fn write(&mut self, value: bool) {
        self.slots[self.position] = value;
    }
    fn move_cursor(&mut self, direction: bool) {
        // false is left, true is right. If needed, the slots vector will be extended.
        if direction {
            if self.position == self.slots.len() - 1 {
                self.slots.push_back(false);
            }
            self.position += 1;
        } else {
            if self.position == 0 {
                self.slots.push_front(false);
            } else {
                self.position -= 1;
            }
        }
    }
    fn diagnostic_checksum(&self) -> u32 {
        let mut count = 0;
        for slot in &self.slots {
            if *slot {
                count += 1;
            }
        }
        count
    }
}

struct Instruction {
    write_value: bool,
    // false is left, true is right.
    move_to: bool,
    next_state: char,
}
impl Instruction {
    fn execute(&self, tape: &mut Tape) -> char {
        tape.write(self.write_value);
        tape.move_cursor(self.move_to);
        self.next_state
    }
}

struct State {
    if_0: Instruction,
    if_1: Instruction,
}

struct Blueprint {
    first_state: char,
    iterations: u32,
    states: FxHashMap<char, State>,
}

fn get_blueprint(input: &str) -> Blueprint {
    let mut state_iter = input.split("In state ");

    // Get the starting state and number of iterations.
    let prelude = state_iter.next().expect("Input should have first section");
    let mut prelude_lines = prelude.lines();
    let mut first_state_iter = prelude_lines
        .next()
        .expect("Input should have first line")
        .chars();
    // Ignore the '.' at the end of the line.
    first_state_iter.next_back();
    let first_state = first_state_iter
        .next_back()
        .expect("First line should have starting state name");
    let mut iterations_iter = prelude_lines
        .next()
        .expect("Input should have second line")
        .split(' ');
    // Ignore the "steps." at the end of the line.
    iterations_iter.next_back();
    let iterations = iterations_iter
        .next_back()
        .expect("Second line should have iterations amount")
        .parse::<u32>()
        .expect("Iterations amount should be a number");

    // Now that state_iter.next() has been called once, the remaining values in it are all state
    // information, so get each state and its instructions.
    let mut states = FxHashMap::default();
    for state in state_iter {
        let mut if_0 = None;
        let mut if_1 = None;
        let mut instruction_iter = state.split("If the current value is ");

        // The first character following "In state " is the state name, so get that.
        let state_name = instruction_iter
            .next()
            .expect("Instruction should have state name")
            .chars()
            .next()
            .expect("State name should have character");

        // Now that instruction_iter.next() has been called once, the remaining values in it are
        // all instruction information, so get that information.
        for instruction in instruction_iter {
            let mut instruction_lines = instruction.lines();
            // Get a mutable reference to whichever instruction matches the current value
            // condition.
            let instruction_ref = if instruction_lines
                .next()
                .expect("Instruction should have first line")
                == "0:"
            {
                &mut if_0
            } else {
                &mut if_1
            };
            // write_value is true if last value of second line is "1." and false otherwise
            // (assuming only other possibility is "0.").
            let write_value = instruction_lines
                .next()
                .expect("Instruction should have second line")
                .split(' ')
                .next_back()
                .expect("Second line should have value")
                == "1.";
            // false is left, true is right.
            let move_to = instruction_lines
                .next()
                .expect("Instruction should have third line")
                .split(' ')
                .next_back()
                .expect("Third line should have movement direction")
                == "right.";
            let next_state = instruction_lines
                .next()
                .expect("Instruction should have fourth line")
                .split(' ')
                .next_back()
                .expect("Fourth line should have next state")
                .chars()
                .next()
                .expect("Next state should have character");

            *instruction_ref = Some(Instruction {
                write_value,
                move_to,
                next_state,
            });
        }
        states.insert(
            state_name,
            State {
                if_0: if_0.expect("First instruction should be defined"),
                if_1: if_1.expect("Second instruction should be defined"),
            },
        );
    }

    Blueprint {
        first_state,
        iterations,
        states,
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
Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A."
            ),
            Solution::U8(3)
        );
    }
}
