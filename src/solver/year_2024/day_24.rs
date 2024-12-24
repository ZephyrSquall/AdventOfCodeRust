use crate::solver::{Solution, Solver};
use rustc_hash::FxHashMap;

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 24,
    title: "Crossed Wires",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    struct Gate<'a, 'b, 'c> {
        operation: GateOperation,
        first_wire: &'a str,
        second_wire: &'b str,
        output_wire: &'c str,
    }
    impl<'a, 'b, 'c> Gate<'a, 'b, 'c> {
        // Attempts to execute the gate's operation on its input wires. If either of the input wires
        // don't have a value yet, this execution fails. If both input wires have a value, this
        // execution succeeds and the result is written to the output wire. This method returns a
        // bool indicating whether execution succeeded so the caller knows whether it needs to check
        // this gate again in the future.
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

    enum GateOperation {
        And,
        Or,
        Xor,
    }

    let mut wire_values = FxHashMap::default();
    let mut pending_gates = Vec::new();

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
        pending_gates.push(Gate {
            operation,
            first_wire,
            second_wire,
            output_wire,
        });
    }

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
