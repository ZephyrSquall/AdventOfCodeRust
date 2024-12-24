use crate::solver::{Solution, Solver};
use itertools::{repeat_n, Itertools};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::max;

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 24,
    title: "Crossed Wires",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let (mut wire_values, mut pending_gates) = get_wire_values_and_gates(input);

    // Continue trying to execute gates until all gates are executed successfully. The retain method
    // removes gates whose execute method returns true, so repeatedly running this method until all
    // gates are removed ensures all gates are executed.
    while !pending_gates.is_empty() {
        pending_gates.retain(|gate| !gate.execute(&mut wire_values));
    }

    // Construct the output from the wires whose names begin with 'z'.
    let mut decimal_output = 0;
    for (wire, wire_value) in wire_values {
        if wire_value && wire.starts_with('z') {
            let wire_index = wire
                .trim_start_matches('z')
                .parse::<u8>()
                .expect("Wire name should only contain digits after the leading 'z' is removed");
            // Take the digit 1 and left-shift it until it is in the proper binary digit position,
            // then add the resulting number to the decimal_output.
            decimal_output += 1 << wire_index;
        }
    }

    Solution::U64(decimal_output)
}

// Strangely, my part 2 current calculates that only six logic gates are incorrect. As it is
// explicitly stated in the puzzle description that exactly eight logic gates are incorrect, this
// means there must be an error in my solution. However, I have extensively checked my logic and
// haven't found any faults, so for now I am stumped as to how I am somehow wrongly verifying at
// least two incorrect gates.
fn solve_2(input: &str) -> Solution {
    let (initial_wire_values, gates) = get_wire_values_and_gates(input);
    // It is useful to have a set of references to each gate, to make calculating differences with
    // other sets of references to gates easier.
    let gate_refs = gates.iter().collect::<FxHashSet<_>>();

    // Get how many binary digits are in the input numbers.
    let mut largest_input_index = 0;
    for initial_wire in initial_wire_values.keys() {
        if initial_wire.starts_with('x') {
            let input_index = initial_wire
                .trim_start_matches('x')
                .parse::<u8>()
                .expect("Wire name should only contain digits after the leading 'z' is removed");
            largest_input_index = max(largest_input_index, input_index);
        }
    }

    // For every gate, we want to check that it gives the correct output digit according to
    // full-adder logic. This means that the output digit for a given position is equal to the XOR
    // of the two input digits for that position and the carry-in digit. The carry-in digit can't be
    // set directly, but it can be guaranteed to be false if every input digit before the current
    // input digits is false (as no carry would ever occur), or guaranteed to be true if every digit
    // before the current input digits is true (as a carry would happen on every digit). In order to
    // confirm whether a given digit is working correctly, all eight combinations of the three input
    // digits are tested and checked to see if they provide the correct output digit. If the digit
    // is correct, all gates used to calculate that digit must have the correct output, so add them
    // to verified gates.
    let mut verified_gates = FxHashSet::default();
    let mut preceding_x_input_wires: Vec<String> = Vec::new();
    let mut preceding_y_input_wires: Vec<String> = Vec::new();
    let mut preceding_output_wires = Vec::new();

    for input_index in 0..=largest_input_index {
        // "{input_index:0>2}" in a format string prints the input_index's value but padded with
        // leading zeroes to be at least two characters wide.
        let x_input_wire = format!("x{input_index:0>2}");
        let y_input_wire = format!("y{input_index:0>2}");
        let output_wire = format!("z{input_index:0>2}");

        // Get every gate that is involved in calculating the next output digit.
        let mut involved_gates = FxHashSet::default();
        let mut involved_outputs = vec![output_wire.as_str()];
        while let Some(output_wire) = involved_outputs.pop() {
            let gate = gates.iter().find(|gate| gate.output_wire == output_wire).expect("Every wire that doesn't begin with x or y should have a corresponding gate that outputs it.");
            involved_gates.insert(gate);

            if !(gate.first_wire.starts_with('x') || gate.first_wire.starts_with('y')) {
                involved_outputs.push(gate.first_wire);
            }
            if !(gate.second_wire.starts_with('x') || gate.second_wire.starts_with('y')) {
                involved_outputs.push(gate.second_wire);
            }
        }

        let mut is_digit_valid = true;
        // repeat_n(iter, 3).multi_cartesian_product gives every 3-length permutation of iter with
        // replacement, which is desired here to generate all 8 possibilities for the two input
        // digits and carry-in digit.
        for values in repeat_n([false, true], 3).multi_cartesian_product() {
            let expected_output_value = if input_index > 0 {
                values[0] ^ values[1] ^ values[2]
            } else {
                // If the input index is zero, disregard the carry bit, as the very first addition
                // uses a half-adder.
                values[0] ^ values[1]
            };

            let mut wire_values = FxHashMap::default();
            wire_values.insert(x_input_wire.as_str(), values[0]);
            wire_values.insert(y_input_wire.as_str(), values[1]);
            // To set the carry-in digit, set every preceding input to the desired carry-in digit to
            // ensure a carry either never happens or always happens.
            for preceding_x_input_wire in &preceding_x_input_wires {
                wire_values.insert(preceding_x_input_wire.as_str(), values[2]);
            }
            for preceding_y_input_wire in &preceding_y_input_wires {
                wire_values.insert(preceding_y_input_wire.as_str(), values[2]);
            }

            // Calculate the output digit.
            let mut pending_gates = involved_gates.clone();
            while !wire_values.contains_key(&output_wire.as_str()) {
                pending_gates.retain(|gate| !gate.execute(&mut wire_values));
            }

            // Test if the output digit is valid. If it is ever wrong for any combination of input
            // digits, then the involved gates must contain incorrect gates.
            if *wire_values
                .get(&output_wire.as_str())
                .expect("This key should have just been inserted in the preceding while loop")
                != expected_output_value
            {
                is_digit_valid = false;
                break;
            }
        }

        if is_digit_valid {
            verified_gates.extend(involved_gates);
        }

        // Do not push the preceding values for the last iteration, as these values need to be
        // checked again for the final carry-out digit.
        if input_index != largest_input_index {
            preceding_x_input_wires.push(x_input_wire);
            preceding_y_input_wires.push(y_input_wire);
        }
        preceding_output_wires.push(output_wire);
    }

    // Calculate the final carry-out digit to ensure it's correct. The inputs are the same as for
    // the final input digit, but the output digit is one index greater.
    let x_input_wire = format!("x{largest_input_index:0>2}");
    let y_input_wire = format!("y{largest_input_index:0>2}");
    let output_wire = format!("z{:0>2}", largest_input_index + 1);

    // Get every gate that is involved in calculating the final carry output digit.
    let mut involved_gates = FxHashSet::default();
    let mut involved_outputs = vec![output_wire.as_str()];
    while let Some(output_wire) = involved_outputs.pop() {
        let gate = gates.iter().find(|gate| gate.output_wire == output_wire).expect("Every wire that doesn't begin with x or y should have a corresponding gate that outputs it.");
        involved_gates.insert(gate);

        if !(gate.first_wire.starts_with('x')
            || gate.first_wire.starts_with('y')
            || gate.first_wire.starts_with('z'))
        {
            involved_outputs.push(gate.first_wire);
        }
        if !(gate.second_wire.starts_with('x')
            || gate.second_wire.starts_with('y')
            || gate.first_wire.starts_with('z'))
        {
            involved_outputs.push(gate.second_wire);
        }
    }

    let mut is_final_carry_valid = true;

    for values in repeat_n([false, true], 3).multi_cartesian_product() {
        let expected_final_carry_value =
            (values[0] && values[1]) || (values[2] && (values[0] ^ values[1]));

        let mut wire_values = FxHashMap::default();
        wire_values.insert(x_input_wire.as_str(), values[0]);
        wire_values.insert(y_input_wire.as_str(), values[1]);
        for preceding_x_input_wire in &preceding_x_input_wires {
            wire_values.insert(preceding_x_input_wire.as_str(), values[2]);
        }
        for preceding_y_input_wire in &preceding_y_input_wires {
            wire_values.insert(preceding_y_input_wire.as_str(), values[2]);
        }
        for preceding_output_wire in &preceding_y_input_wires {
            wire_values.insert(preceding_output_wire.as_str(), values[2]);
        }

        let mut pending_gates = involved_gates.clone();

        while !wire_values.contains_key(&output_wire.as_str()) {
            pending_gates.retain(|gate| !gate.execute(&mut wire_values));
        }

        if *wire_values
            .get(&output_wire.as_str())
            .expect("This key should have just been inserted in the preceding while loop")
            != expected_final_carry_value
        {
            is_final_carry_valid = false;
            break;
        }
    }

    if is_final_carry_valid {
        verified_gates.extend(involved_gates);
    }

    // As this answer is obviously incorrect for showing the wrong number of gates, for now I simply
    // print the list of gates and don't bother with sorting them for my final puzzle output.
    for gate in gate_refs.difference(&verified_gates) {
        println!("Potential wrong gate: {gate:?}");
    }

    Solution::U64(0)
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Gate<'a, 'b, 'c> {
    operation: GateOperation,
    first_wire: &'a str,
    second_wire: &'b str,
    output_wire: &'c str,
}
impl<'a, 'b, 'c> Gate<'a, 'b, 'c> {
    // Attempts to execute the gate's operation on its input wires. If either of the input wires
    // don't have a value yet, this execution fails. If both input wires have a value, this
    // execution succeeds and the result is written to the output wire. This method returns a bool
    // indicating whether execution succeeded so the caller knows whether it needs to check this
    // gate again in the future.
    fn execute(&self, wire_values: &mut FxHashMap<&'c str, bool>) -> bool {
        if let Some((first_value, second_value)) = wire_values
            .get(&self.first_wire)
            .zip(wire_values.get(&self.second_wire))
        {
            let output_value = match self.operation {
                GateOperation::And => *first_value && *second_value,
                GateOperation::Or => *first_value || *second_value,
                GateOperation::Xor => *first_value ^ *second_value,
            };
            wire_values.insert(self.output_wire, output_value);
            return true;
        }
        false
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum GateOperation {
    And,
    Or,
    Xor,
}

fn get_wire_values_and_gates(input: &str) -> (FxHashMap<&str, bool>, FxHashSet<Gate>) {
    let mut wire_values = FxHashMap::default();
    let mut gates = FxHashSet::default();
    let mut line_iter = input.lines();

    // Get the wires with an initial value and insert them into the wire_values map. Stop iterating
    // when a blank line is reached, as this indicates that the input is swapping to describing
    // gates. Use by_ref() to borrow line_iter instead of consuming it, so it can be reused in a
    // later for loop.
    for line in line_iter.by_ref() {
        if line.is_empty() {
            break;
        }

        let mut str_iter = line.split(": ");
        let wire = str_iter.next().expect("Line should have first value");
        // Assuming that the second string can only be "0" or "1", set value to whether this second
        // string equals "1".
        let value = str_iter.next().expect("Line should have second value") == "1";
        wire_values.insert(wire, value);
    }

    // Get each gate's operation, input wire names, and output wire name. Insert everything into the
    // pending_gates vector (as no gate has been executed yet so every gate is pending).
    for line in line_iter {
        let mut outer_str_iter = line.split(" -> ");
        let mut inner_str_iter = outer_str_iter
            .next()
            .expect("Line should have values before the \" -> \"")
            .split(' ');
        let first_wire = inner_str_iter
            .next()
            .expect("Line should have first value before the \" -> \"");
        let operation = match inner_str_iter
            .next()
            .expect("Line should have second value before the \" -> \"")
        {
            "AND" => GateOperation::And,
            "OR" => GateOperation::Or,
            "XOR" => GateOperation::Xor,
            _ => panic!("Unsupported gate operation encountered"),
        };
        let second_wire = inner_str_iter
            .next()
            .expect("Line should have third value before the \" -> \"");
        let output_wire = outer_str_iter
            .next()
            .expect("Line should have value after the \" -> \"");
        gates.insert(Gate {
            operation,
            first_wire,
            second_wire,
            output_wire,
        });
    }

    (wire_values, gates)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"
            ),
            Solution::U8(4)
        );
    }
    #[test]
    fn example1_2() {
        assert_eq!(
            solve_1(
                "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"
            ),
            Solution::U16(2024)
        );
    }
}
