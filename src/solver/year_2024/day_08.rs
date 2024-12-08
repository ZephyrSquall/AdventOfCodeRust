use crate::solver::{Solution, Solver};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 8,
    title: "Resonant Collinearity",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    // Given a vector of the positions of all antennas of one particular frequency, find the
    // position of all antinodes and insert them into the set of antinodes.
    fn find_antinodes_for_frequency(
        antennas_with_frequency: &[Position],
        antinodes: &mut FxHashSet<Position>,
        x_len: usize,
        y_len: usize,
    ) {
        // Iterate over every pair of antennas.
        for (antenna_1, antenna_2) in antennas_with_frequency.iter().tuple_combinations() {
            // Get the difference between the first and second position (as a mathematical vector
            // pointing from the first position to the second position).
            let dx = antenna_2.x - antenna_1.x;
            let dy = antenna_2.y - antenna_1.y;
            // The difference points from the first position to the second position, so the two
            // antinodes are obtained by subtracting the difference from the first position and
            // adding it to the second position.
            let antinode_1 = Position {
                x: antenna_1.x - dx,
                y: antenna_1.y - dy,
            };
            let antinode_2 = Position {
                x: antenna_2.x + dx,
                y: antenna_2.y + dy,
            };
            // For each antinode, if it is in bounds, add it to the set of antinodes. There is no
            // need to check for duplicate values because sets only allow unique values.
            if antinode_1.is_within_bounds(x_len, y_len) {
                antinodes.insert(antinode_1);
            }
            if antinode_2.is_within_bounds(x_len, y_len) {
                antinodes.insert(antinode_2);
            }
        }
    }

    let (antennas, x_len, y_len) = get_antennas_and_lens(input);
    let mut antinodes = FxHashSet::default();

    for antennas_with_frequency in antennas.values() {
        find_antinodes_for_frequency(antennas_with_frequency, &mut antinodes, x_len, y_len);
    }

    Solution::USize(antinodes.len())
}

fn solve_2(input: &str) -> Solution {
    // The only difference in part two is updating find_antinodes_for_frequency to find every
    // resonant frequency.
    fn find_antinodes_for_frequency(
        antennas_with_frequency: &[Position],
        antinodes: &mut FxHashSet<Position>,
        x_len: usize,
        y_len: usize,
    ) {
        // Frequencies that only have a single antenna will be skipped as they have zero pairs to
        // iterate over, thus no antinodes will be assigned to such antennas.
        for (antenna_1, antenna_2) in antennas_with_frequency.iter().tuple_combinations() {
            let dx = antenna_2.x - antenna_1.x;
            let dy = antenna_2.y - antenna_1.y;

            // The difference vector is multiplied by the number of loop repetitions until an
            // out-of-bounds antinode is found to get antinodes at every resonant frequency. By
            // starting the repetitions at 0, the first antinodes will be found on top of the
            // antennas.
            let mut repetitions = 0;
            loop {
                let antinode = Position {
                    x: antenna_1.x - (dx * repetitions),
                    y: antenna_1.y - (dy * repetitions),
                };
                if antinode.is_within_bounds(x_len, y_len) {
                    antinodes.insert(antinode);
                    repetitions += 1;
                } else {
                    break;
                }
            }

            repetitions = 0;
            loop {
                let antinode = Position {
                    x: antenna_2.x + (dx * repetitions),
                    y: antenna_2.y + (dy * repetitions),
                };
                if antinode.is_within_bounds(x_len, y_len) {
                    antinodes.insert(antinode);
                    repetitions += 1;
                } else {
                    break;
                }
            }
        }
    }

    let (antennas, x_len, y_len) = get_antennas_and_lens(input);
    let mut antinodes = FxHashSet::default();

    for antennas_with_frequency in antennas.values() {
        find_antinodes_for_frequency(antennas_with_frequency, &mut antinodes, x_len, y_len);
    }

    Solution::USize(antinodes.len())
}

#[derive(PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}
impl Position {
    // It is assumed that the puzzle input isn't big enough to cause issues with converting usize to
    // isize (it will only wrap around if the input grid has isize::MAX rows or columns).
    #[allow(clippy::cast_possible_wrap)]
    fn is_within_bounds(&self, x_len: usize, y_len: usize) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < x_len as isize && self.y < y_len as isize
    }
}

fn get_antennas_and_lens(input: &str) -> (FxHashMap<char, Vec<Position>>, usize, usize) {
    // antennas is a hash map of char keys and vec<Position> values. Each value vector gives the
    // position of every antenna of the corresponding key character.
    let mut antennas = FxHashMap::default();
    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character != '.' {
                // Get the list of antenna positions for antennas matching the current character. If
                // there are no antennas, first insert an empty list. Then add the current antenna
                // position to the list.
                #[allow(clippy::cast_possible_wrap)]
                antennas
                    .entry(character)
                    .or_insert(Vec::new())
                    .push(Position {
                        x: x as isize,
                        y: y as isize,
                    });
            }
        }
    }

    let x_len = input
        .lines()
        .next()
        .expect("Input should have first line")
        .chars()
        .count();
    let y_len = input.lines().count();

    (antennas, x_len, y_len)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            ),
            Solution::U8(14)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            ),
            Solution::U8(34)
        );
    }
}
