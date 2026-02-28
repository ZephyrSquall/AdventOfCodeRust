use crate::solver::{AdventOfCode, Solution};
use rustc_hash::FxHashMap;
use std::collections::BTreeSet;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2025,
    day: 7,
    title: "Laboratories",
    part_solvers: &[solve_1, solve_2],
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
        // The tachyon_beam_positions must be cloned since positions must be added and removed to
        // the outgoing tachyon_beam_positions while still iterating over the incoming
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

fn solve_2(input: &str) -> Solution {
    // Unlike in part 1, beam position must now include vertical position as well, as the recursive
    // nature of this solution does not permit simply going down the tachyon manifold one time.
    #[derive(PartialEq, Eq, Hash, Clone)]
    struct Position {
        x: usize,
        y: usize,
    }

    fn split_timeline(
        timeline_cache: &mut FxHashMap<Position, u64>,
        tachyon_manifold: &Vec<Vec<bool>>,
        starting_position: &Position,
    ) -> u64 {
        // If the number of timelines from this position has previously been calculated, reuse that
        // value.
        if let Some(timelines) = timeline_cache.get(starting_position) {
            return *timelines;
        }

        for y in starting_position.y..tachyon_manifold.len() {
            if tachyon_manifold[y][starting_position.x] {
                // Upon finding a splitter, recursively call this function again for positions one
                // left and one right of the splitter.
                let timelines = split_timeline(
                    timeline_cache,
                    tachyon_manifold,
                    &Position {
                        x: starting_position.x - 1,
                        y,
                    },
                ) + split_timeline(
                    timeline_cache,
                    tachyon_manifold,
                    &Position {
                        x: starting_position.x + 1,
                        y,
                    },
                );
                timeline_cache.insert(starting_position.clone(), timelines);
                return timelines;
            }
        }

        timeline_cache.insert(starting_position.clone(), 1);
        1
    }

    let starting_line = input.lines().next().expect("Input should have first line");
    let starting_tachyon_beam_position = Position {
        x: starting_line
            .chars()
            .position(|char| char == 'S')
            .expect("Starting line should contain an 'S'"),
        y: 0,
    };

    // tachyon_manifold is a 2D grid of bools where true indicates a splitter and false indicates
    // free space (or the starting position).
    let tachyon_manifold = input
        .lines()
        .map(|line| line.chars().map(|char| char == '^').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Map to store the memoized results of recursive calls.
    let mut timeline_cache = FxHashMap::default();

    Solution::U64(split_timeline(
        &mut timeline_cache,
        &tachyon_manifold,
        &starting_tachyon_beam_position,
    ))
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

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
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
            Solution::U8(40)
        );
    }
}
