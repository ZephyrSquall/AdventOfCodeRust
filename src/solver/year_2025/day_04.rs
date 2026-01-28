use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2025,
    day: 4,
    title: "Printing Department",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|character| character == '@')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut accessible_rolls_of_paper = 0;

    // For each grid point,
    for y in 0..grid.len() {
        'next: for x in 0..grid[0].len() {
            // If it is a role of paper,
            if grid[y][x] {
                let mut adjacent_rolls_of_paper = 0;
                // Check its eight neighbours (use saturating_sub to avoid overflow when y == 0 or
                // x == 0),
                for other_y in y.saturating_sub(1)..=y + 1 {
                    for other_x in x.saturating_sub(1)..=x + 1 {
                        // Do not check itself,
                        if !(other_y == y && other_x == x)
                            && grid
                                // Use get method to check if the index is in bounds.
                                .get(other_y)
                                .is_some_and(|row| row.get(other_x) == Some(&true))
                        {
                            adjacent_rolls_of_paper += 1;
                            // Skip to the next grid point if too many roles of paper are found.
                            if adjacent_rolls_of_paper >= 4 {
                                continue 'next;
                            }
                        }
                    }
                }
                // This won't be reached if we skipped to the next grid point due to finding 4
                // adjacent roles of paper.
                accessible_rolls_of_paper += 1;
            }
        }
    }

    Solution::U16(accessible_rolls_of_paper)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            ),
            Solution::U8(13)
        );
    }
}
