use crate::solver::{AdventOfCode, Solution};
use std::collections::BTreeSet;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2025,
    day: 7,
    title: "Laboratories",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    // A BTreeSet automatically sorts itself and deduplicates elements, removing the need to do
    // these manually.
    let mut tachyon_beam_positions = BTreeSet::new();
    let mut splits = 0;
    let mut lines = input.lines();

    let starting_line = lines.next().expect("Input should have first line");
    let starting_tachyon_beam_position = starting_line
        .chars()
        .position(|char| char == 'S')
        .expect("Starting line should contain an 'S'");
    tachyon_beam_positions.insert(starting_tachyon_beam_position);

    for line in lines {
        // The tachyon_beam_positions must be cloned as positions must be added and removed to the
        // outgoing tachyon_beam_positions while still iterating over the incoming
        // tachyon_beam_positions.
        let incoming_tachyon_beam_positions = tachyon_beam_positions.clone().into_iter();

        for incoming_tachyon_beam_position in incoming_tachyon_beam_positions {
            if line
                .chars()
                .nth(incoming_tachyon_beam_position)
                .expect("Beam positions should remain with the line")
                == '^'
            {
                tachyon_beam_positions.remove(&incoming_tachyon_beam_position);
                tachyon_beam_positions.insert(incoming_tachyon_beam_position - 1);
                tachyon_beam_positions.insert(incoming_tachyon_beam_position + 1);
                splits += 1;
            }
        }
    }

    Solution::U32(splits)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            ),
            Solution::U8(21)
        );
    }
}
