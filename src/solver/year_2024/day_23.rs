use crate::solver::{Solution, Solver};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter, Result},
};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 23,
    title: "LAN Party",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
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

fn solve_2(input: &str) -> Solution {
    // Every computer has a key in this map, and the value is a vector of all other computers
    // directly connected to that computer.
    let mut direct_connections_from_computer = FxHashMap::default();
    // This is a set of sets. Each of the inner sets describes a group of computers where every
    // computer in the group has a connection to every other computer, and the outer set holds every
    // such group of computers fully connected to each other that has been found so far. By using
    // sets, if the same fully connected group if formed multiple different ways, they are still
    // only stored once.
    let mut fully_connected_groups: FxHashSet<BTreeSet<Computer>> = FxHashSet::default();

    for line in input.lines() {
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

        // Add each computer to each other's list of directly-connected computers.
        direct_connections_from_computer
            .entry(first_computer.clone())
            .and_modify(|connected_computers: &mut Vec<Computer>| {
                connected_computers.push(second_computer.clone());
            })
            .or_insert(vec![second_computer.clone()]);
        direct_connections_from_computer
            .entry(second_computer.clone())
            .and_modify(|connected_computers: &mut Vec<Computer>| {
                connected_computers.push(first_computer.clone());
            })
            .or_insert(vec![first_computer.clone()]);

        // Entries mutably borrow the hashmap, but now we want to take two immutable borrows at the
        // same time so this map doesn't need to be searched on every iteration of the following for
        // loop. Hence the above mutable borrows must be dropped so the following two immutable
        // borrows can be made.
        let direct_connections_from_first_computer = direct_connections_from_computer
            .get(&first_computer)
            .expect("First computer should be in hash map as it was just inserted");
        let direct_connections_from_second_computer = direct_connections_from_computer
            .get(&second_computer)
            .expect("Second computer should be in hash map as it was just inserted");

        // For each fully connected group found so far and for each computer, check if that computer
        // is already in the fully connected group, and if not, check if the computer has a
        // connection to every computer in the fully connected group. If so, a new fully connected
        // group can be formed by taking the current fully connected group and adding the computer
        // to it.
        let mut new_fully_connected_groups = Vec::new();

        for fully_connected_group in &fully_connected_groups {
            if fully_connected_group.iter().all(|computer| {
                *computer != first_computer
                    && direct_connections_from_first_computer.contains(computer)
            }) {
                let mut new_fully_connected_group = fully_connected_group.clone();
                new_fully_connected_group.insert(first_computer.clone());
                new_fully_connected_groups.push(new_fully_connected_group);
            }
            if fully_connected_group.iter().all(|computer| {
                *computer != second_computer
                    && direct_connections_from_second_computer.contains(computer)
            }) {
                let mut new_fully_connected_group = fully_connected_group.clone();
                new_fully_connected_group.insert(second_computer.clone());
                new_fully_connected_groups.push(new_fully_connected_group);
            }
        }

        for new_fully_connected_group in new_fully_connected_groups {
            fully_connected_groups.insert(new_fully_connected_group);
        }

        // Create a new fully-connected group containing just the two computers.
        let mut new_fully_connected_group = BTreeSet::new();
        new_fully_connected_group.insert(first_computer);
        new_fully_connected_group.insert(second_computer);
        fully_connected_groups.insert(new_fully_connected_group);
    }

    // All fully connected groups have been found, so take the biggest one and get the password from
    // it. There is no need to sort the computers before combining their names into the password, as
    // BTreeSet automatically sorts its elements according to its Ord trait, and the Computer's
    // derived Ord trait automatically orders the computers alphabetically.
    let password = fully_connected_groups
        .into_iter()
        // Get the fully connected group with the maximum length. It is assumed there is only one
        // such group, as otherwise the problem would not have a single unambiguous answer.
        .max_by_key(BTreeSet::len)
        .expect("The set of fully connected groups should have at least one element")
        // The largest fully connected group has been obtained and all groups are presorted
        // alphabetically. So simply join the computers together into one string. The join method
        // turns the Computers into strings using their Display trait, which is manually implemented
        // to be a string containing the computer's two characters in order.
        .into_iter()
        .join(",");

    Solution::String(password)
}

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
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
// To turn a Computer into a string, simply put its two characters side by side.
impl Display for Computer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}", self.first_char, self.second_char)
    }
}

struct DirectConnection {
    first: Computer,
    second: Computer,
}
impl DirectConnection {
    // Given two other computers, if either of them connect to one of the computers in this direct
    // connection, return the other computer in this direct connection. If neither of the two given
    // computers connect to either of the computers in this direct connection, return None. It is
    // assumed the puzzle input has no duplicate edges, which means it's impossible for both of the
    // given computers to match a computer in the direct connection at the same time.
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

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
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
            Solution::Str("co,de,ka,ta")
        );
    }
}
