use crate::solver::{AdventOfCode, Solution};
use rustc_hash::FxHashMap;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2023,
    day: 8,
    title: "Haunted Wasteland",
    part_solvers: &[solve_1],
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

fn solve_1(input: &str) -> Solution {
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

    let mut steps = 0;
    let mut current_node = "AAA";
    'outer: loop {
        for direction in &directions {
            steps += 1;
            current_node = node_map
                .get(current_node)
                .expect("All nodes should exist in node_map")
                .next_node(direction);

            if current_node == "ZZZ" {
                break 'outer;
            }
        }
    }

    Solution::U32(steps)
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
}
