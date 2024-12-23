use crate::solver::{Solution, Solver};
use rustc_hash::FxHashMap;

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 23,
    title: "LAN Party",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    struct Computer {
        first_char: char,
        second_char: char,
    }
    impl Computer {
        fn new(letters: &str) -> Computer {
            let mut letter_iter = letters.chars();
            let first_char = letter_iter
                .next()
                .expect("Computer should have first letter");
            let second_char = letter_iter
                .next()
                .expect("Computer should have second letter");
            Computer {
                first_char,
                second_char,
            }
        }
        fn starts_with_t(&self) -> bool {
            self.first_char == 't'
        }
    }

    #[derive(Debug)]
    struct DirectConnection {
        first: Computer,
        second: Computer,
    }
    impl DirectConnection {
        // Given two other computers, if either of them connect to one of the computers in this
        // direct connection, return the other computer in this direct connection. If neither of the
        // two given computers connect to either of the computers in this direct connection, return
        // None. It is assumed the puzzle input has no duplicate edges, which means it's impossible
        // for both of the given computers to match a computer in the direct connection at the same
        // time.
        fn get_other_computer(
            &self,
            first_computer: &Computer,
            second_computer: &Computer,
        ) -> Option<&Computer> {
            if self.first == *first_computer || self.first == *second_computer {
                Some(&self.second)
            } else if self.second == *first_computer || self.second == *second_computer {
                Some(&self.first)
            } else {
                None
            }
        }
    }

    let mut direct_connections: Vec<DirectConnection> = Vec::new();
    let mut triangles_containing_computers_starting_with_t = 0;

    for line in input.lines() {
        // Get the computers in the next direct connection to be added.
        let mut computer_iter = line.split('-');
        let first_computer = Computer::new(
            computer_iter
                .next()
                .expect("Line should have first computer"),
        );
        let second_computer = Computer::new(
            computer_iter
                .next()
                .expect("Line should have second computer"),
        );

        // Keep a count of how many times each other computer is found directly connected to either
        // first_computer or second_computer.
        let mut other_computer_counts = FxHashMap::default();

        for direct_connection in &direct_connections {
            if let Some(other_computer) =
                direct_connection.get_other_computer(&first_computer, &second_computer)
            {
                other_computer_counts
                    .entry(other_computer)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        // Go through each count and find where it equals 2, indicating a set of three has been
        // found (a computer is counted twice if it is connected to both first_computer and
        // second_computer). If so, check if any of the computers in the set of three start with t,
        // and if so, increment the count of such sets by 1. Note that no set will be double-counted
        // because it will only be found when its last connection is being added; the count can't
        // reach 2 beforehand.
        for (other_computer, count) in other_computer_counts {
            if count == 2
                && (first_computer.starts_with_t()
                    || second_computer.starts_with_t()
                    || other_computer.starts_with_t())
            {
                triangles_containing_computers_starting_with_t += 1;
            }
        }

        // Add the new direct connection. This is done at the end of the loop iteration to prevent
        // this edge from interfering with the counts.
        direct_connections.push(DirectConnection {
            first: first_computer,
            second: second_computer,
        });
    }

    Solution::U32(triangles_containing_computers_starting_with_t)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"
            ),
            Solution::U8(7)
        );
    }
}
