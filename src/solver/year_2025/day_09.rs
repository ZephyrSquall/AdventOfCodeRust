use crate::solver::{AdventOfCode, Solution};
use itertools::Itertools;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2025,
    day: 9,
    title: "Movie Theater",
    part_solvers: &[solve_1],
};

struct Position {
    x: u64,
    y: u64,
}
impl Position {
    fn rectangle_area_with(&self, other: &Position) -> u64 {
        // There is a subtle fencepost issue here, as we are not strictly getting the area between
        // two points on a cartesian plane (measuring the fence), but rather the number of floor
        // tiles which are contained between the two positions on a tiled grid including start and
        // end positions (measuring the posts). We must ensure the side lengths include both the
        // starting row/column and the ending row/column of the rectangle on this grid. Without the
        // + 1, each rectangle would be missing one row and one column.
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

fn solve_1(input: &str) -> Solution {
    let red_tile_positions = input
        .lines()
        .map(|line| {
            let num_strs = line.split_once(',').expect("Line should have a comma");
            let first_num = num_strs
                .0
                .parse()
                .expect("First string should be a valid number");
            let second_num = num_strs
                .1
                .parse()
                .expect("Second string should be a valid number");
            Position {
                x: first_num,
                y: second_num,
            }
        })
        .collect::<Vec<_>>();

    let largest_rectangle = red_tile_positions
        .iter()
        .combinations(2)
        // We now have every possible combination of two tiles. Map these combinations to the area
        // of the rectangle between them and get the largest area.
        .map(|positions| positions[0].rectangle_area_with(positions[1]))
        .max()
        .expect("There should be at least one combination of red tile positions");

    Solution::U64(largest_rectangle)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
            ),
            Solution::U8(50)
        );
    }
}
