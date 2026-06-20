use crate::solver::{AdventOfCode, Solution};
use rustc_hash::FxHashMap;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2023,
    day: 8,
    title: "Haunted Wasteland",
    part_solvers: &[solve_1, solve_2],
};

enum Direction {
    Left,
    Right,
}
impl Direction {
    fn new(character: char) -> Direction {
        match character {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid character"),
        }
    }
}

struct Node<'a, 'b> {
    left: &'a str,
    right: &'b str,
}
impl Node<'_, '_> {
    fn next_node(&self, direction: &Direction) -> &str {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

fn get_directions_and_node_map(input: &str) -> (Vec<Direction>, FxHashMap<&str, Node<'_, '_>>) {
    let mut line_iter = input.lines();
    let directions = line_iter
        .next()
        .expect("Input should have first line")
        .chars()
        .map(Direction::new)
        .collect::<Vec<_>>();

    // Ignore the empty line between the directions and the list of nodes.
    line_iter.next();

    let node_map = line_iter
        .map(|line| {
            let (source_str, mut destinations_str) = line
                .split_once(" = (")
                .expect("All node lines should have substring \" = (\"");

            destinations_str = destinations_str
                .strip_suffix(')')
                .expect("All node lines should end with ')'");
            let (left_destination_str, right_destination_str) = destinations_str
                .split_once(", ")
                .expect("All lines should have substring \", \"");

            // This tuple is the key-value pair to be collected into node_map.
            (
                source_str,
                Node {
                    left: left_destination_str,
                    right: right_destination_str,
                },
            )
        })
        .collect::<FxHashMap<_, _>>();

    (directions, node_map)
}

fn solve_1(input: &str) -> Solution {
    let (directions, node_map) = get_directions_and_node_map(input);

    let mut step_count = 0;
    let mut current_node = "AAA";
    for direction in directions.iter().cycle() {
        step_count += 1;
        current_node = node_map
            .get(current_node)
            .expect("All nodes should exist in node_map")
            .next_node(direction);

        if current_node == "ZZZ" {
            break;
        }
    }

    Solution::U32(step_count)
}

fn solve_2(input: &str) -> Solution {
    // To the best of my knowledge, if using the information in the puzzle description alone, there
    // is no solution more efficient than brute force i.e. traversing all paths simultaneously until
    // every path simultaneously falls on a node ending with 'Z'. However, the number of steps
    // required is so huge that such a brute force solution would take at least several hours to
    // run, so this can't be the intended solution.
    //
    // Analyzing the puzzle input reveals several patterns in the node structure which the puzzle
    // description does not mention. As brute force can't be the intended solution, it seems instead
    // that the intention of this puzzle is to discover these patterns in the puzzle input and take
    // advantage of them. As such, my solution takes advantage of these patterns, and therefore
    // assumes every puzzle input follows them. These patterns are:
    // - Every node ending with 'Z' is part of a loop that contains no other nodes ending with 'Z'.
    // - The amount of steps required to reach the node ending with 'Z' again is always consistent.
    // - Every node ending with 'A' is not directly part of these loops, i.e. they will never be
    //     visited again after navigating away from them.
    // - The first step from each node ending with 'A' goes to the node in the loop immediately
    //     after the node ending with 'Z'.
    //
    // This means each loop effectively starts on the node ending with 'Z', since by having the 'A'
    // nodes point to the second node in each loop, the number of steps from the 'A' node to the 'Z'
    // node is the same as from the 'Z' node back to itself again. This observation reduces this
    // problem from counting steps to determining when the loop cycles all line up again. This will
    // occur on the lowest common multiple of the length of all individual loops.

    fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }

        a
    }

    fn lcm(a: u64, b: u64) -> u64 {
        a * b / gcd(a, b)
    }

    let (directions, node_map) = get_directions_and_node_map(input);

    let step_count =
    // Create an iterator of node names ending with 'A'
    node_map
        .keys()
        .filter(|node_str| node_str.ends_with('A'))
        // Map each node ending with 'A' to the step count until finding the node ending with 'Z'
        .map(|starting_node| {
            let mut step_count = 0;
            let mut current_node = *starting_node;
            for direction in directions.iter().cycle() {
                step_count += 1;
                current_node = node_map
                    .get(current_node)
                    .expect("All nodes should exist in node_map")
                    .next_node(direction);
                if current_node.ends_with('Z') {
                    break;
                }
            }
            step_count
        })
        // Find the lowest common multiple of all step counts
        .reduce(lcm)
        .expect("Iterator should have at least one element");

    Solution::U64(step_count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            Solution::U8(2)
        );
    }
    #[test]
    fn example1_2() {
        assert_eq!(
            solve_1(
                "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            Solution::U8(6)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            Solution::U8(6)
        );
    }
}
