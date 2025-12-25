use crate::solver::{AdventOfCode, Solution};
use rustc_hash::FxHashMap;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2015,
    day: 7,
    title: "Some Assembly Required",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let mut wires = FxHashMap::default();
    let mut gates = get_gates(input);

    while !gates.is_empty() {
        // We want to run the execute method on every gate in the Vector exactly once, and remove
        // every gate who returned true (indicating it successfully executed its operation). This is
        // exactly what the vector's retain() method does (except it removes elements that return
        // false so the return value must be flipped with !).
        gates.retain(|gate| !gate.execute(&mut wires));
    }

    Solution::U16(*wires.get("a").expect("Should have a value for wire \"a\""))
}

fn solve_2(input: &str) -> Solution {
    let mut wires = FxHashMap::default();
    let mut gates = get_gates(input);

    // Remove the gate that assigns a value to wire "b"
    gates.retain(|gate| {
        if let Gate::Assign(_value_or_identifier, output) = gate {
            *output != "b"
        } else {
            true
        }
    });
    // Get the solution from part 1.
    let Solution::U16(wire_b_override) = solve_1(input) else {
        panic!("Part 1 should return a u16")
    };
    // Add a new gate that assigns the solution from part 1 to wire "b".
    gates.push(Gate::Assign(ValueOrIdentifier::Value(wire_b_override), "b"));

    while !gates.is_empty() {
        gates.retain(|gate| !gate.execute(&mut wires));
    }

    Solution::U16(*wires.get("a").expect("Should have a value for wire \"a\""))
}

// Represents an argument to a gate that can either be a literal value or an identifier for a wire.
enum ValueOrIdentifier<'a> {
    Value(u16),
    Identifier(&'a str),
}
impl<'a> ValueOrIdentifier<'a> {
    // If the input string can be converted to an integer, it is interpreted as a literal value and
    // stored as an integer. Otherwise, it is interpreted as an identifier and the original input
    // string is stored as string.
    fn new(value_or_identifier: &'a str) -> Self {
        if let Ok(value) = value_or_identifier.parse() {
            ValueOrIdentifier::Value(value)
        } else {
            ValueOrIdentifier::Identifier(value_or_identifier)
        }
    }
    // Attempts to get the value. If unsuccessful (is an Identifier which hasn't yet been given a
    // value in the wires hash map), returns None.
    fn get_value<'b>(&'b self, wires: &'b FxHashMap<&str, u16>) -> Option<&'b u16> {
        match self {
            ValueOrIdentifier::Value(value) => Some(value),
            ValueOrIdentifier::Identifier(identifier) => wires.get(identifier),
        }
    }
}

enum Gate<'a, 'b, 'c> {
    Assign(ValueOrIdentifier<'a>, &'c str),
    Not(ValueOrIdentifier<'a>, &'c str),
    And(ValueOrIdentifier<'a>, ValueOrIdentifier<'b>, &'c str),
    Or(ValueOrIdentifier<'a>, ValueOrIdentifier<'b>, &'c str),
    LShift(ValueOrIdentifier<'a>, ValueOrIdentifier<'b>, &'c str),
    RShift(ValueOrIdentifier<'a>, ValueOrIdentifier<'b>, &'c str),
}
impl<'c> Gate<'_, '_, 'c> {
    // Attempts to execute the operation of the gate on its arguments. If at least one argument is
    // an identifier that doesn't yet have a value in the wires hash map, the execution fails and
    // this method returns false. Otherwise, the operation is carried out, the result is stored in
    // the wires hash map under the name of the output identifier, and this method returns true.
    fn execute(&self, wires: &mut FxHashMap<&'c str, u16>) -> bool {
        match self {
            Gate::Assign(arg, output) => {
                let arg = arg.get_value(wires);
                match arg {
                    Some(value) => {
                        wires.insert(output, *value);
                        true
                    }
                    None => false,
                }
            }
            Gate::Not(arg, output) => {
                let arg = arg.get_value(wires);
                match arg {
                    Some(value) => {
                        wires.insert(output, !*value);
                        true
                    }
                    None => false,
                }
            }
            Gate::And(arg_1, arg_2, output) => {
                let arg_1 = arg_1.get_value(wires);
                let arg_2 = arg_2.get_value(wires);
                match (arg_1, arg_2) {
                    (Some(value1), Some(value2)) => {
                        wires.insert(output, *value1 & *value2);
                        true
                    }
                    _ => false,
                }
            }
            Gate::Or(arg_1, arg_2, output) => {
                let arg_1 = arg_1.get_value(wires);
                let arg_2 = arg_2.get_value(wires);
                match (arg_1, arg_2) {
                    (Some(value1), Some(value2)) => {
                        wires.insert(output, *value1 | *value2);
                        true
                    }
                    _ => false,
                }
            }
            Gate::LShift(arg_1, arg_2, output) => {
                let arg_1 = arg_1.get_value(wires);
                let arg_2 = arg_2.get_value(wires);
                match (arg_1, arg_2) {
                    (Some(value1), Some(value2)) => {
                        wires.insert(output, *value1 << *value2);
                        true
                    }
                    _ => false,
                }
            }
            Gate::RShift(arg_1, arg_2, output) => {
                let arg_1 = arg_1.get_value(wires);
                let arg_2 = arg_2.get_value(wires);
                match (arg_1, arg_2) {
                    (Some(value1), Some(value2)) => {
                        wires.insert(output, *value1 >> *value2);
                        true
                    }
                    _ => false,
                }
            }
        }
    }
}

fn get_gates(input: &str) -> Vec<Gate<'_, '_, '_>> {
    let mut gates = Vec::new();

    for line in input.lines() {
        let mut word_iter = line.split(' ');
        let first_word = word_iter
            .next()
            .expect("All lines should have at least one word");

        if first_word == "NOT" {
            let first_argument = word_iter.next().expect("NOT gate should have argument");
            let first_argument = ValueOrIdentifier::new(first_argument);
            // Ignore the "->"
            word_iter.next();
            let output = word_iter
                .next()
                .expect("NOT gate should have output identifier");
            gates.push(Gate::Not(first_argument, output));
        } else {
            let first_argument = ValueOrIdentifier::new(first_word);

            let gate = word_iter
                .next()
                .expect("Line should have at least two words");
            if gate == "->" {
                let output = word_iter
                    .next()
                    .expect("ASSIGN gate should have output identifier");
                gates.push(Gate::Assign(first_argument, output));
            } else {
                let second_argument = word_iter.next().expect("Gate should have second argument");
                let second_argument = ValueOrIdentifier::new(second_argument);
                // Ignore the "->"
                word_iter.next();
                let output = word_iter
                    .next()
                    .expect("Gate should have output identifier");

                match gate {
                    "AND" => gates.push(Gate::And(first_argument, second_argument, output)),
                    "OR" => gates.push(Gate::Or(first_argument, second_argument, output)),
                    "LSHIFT" => gates.push(Gate::LShift(first_argument, second_argument, output)),
                    "RSHIFT" => gates.push(Gate::RShift(first_argument, second_argument, output)),
                    _ => panic!("Invalid gate"),
                }
            }
        }
    }

    gates
}

#[cfg(test)]
mod test {
    use super::*;

    // In this example taken from the problem, there is no wire "a", so wire "i" is renamed to "a"
    // to give this test something to assert.
    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> a"
            ),
            Solution::U16(65079)
        );
    }
}
