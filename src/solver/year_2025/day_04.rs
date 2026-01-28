use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2025,
    day: 4,
    title: "Printing Department",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let grid = get_grid(input);
    let mut accessible_rolls_of_paper = 0;

    // For each grid point,
    for (y, grid_row) in grid.iter().enumerate() {
        for (x, roll_of_paper) in grid_row.iter().enumerate() {
            // If it is a roll of paper and it is accessible,
            if *roll_of_paper && is_roll_of_paper_accessible(&grid, x, y) {
                accessible_rolls_of_paper += 1;
            }
        }
    }

    Solution::U16(accessible_rolls_of_paper)
}

fn solve_2(input: &str) -> Solution {
    let mut grid = get_grid(input);
    let mut removed_rolls_of_paper = 0;
    let mut is_unchanged = false;

    // Keep checking the grid, removing all accessible rolls of paper, until a full pass over the
    // grid results in no changes.
    while !is_unchanged {
        is_unchanged = true;
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] && is_roll_of_paper_accessible(&grid, x, y) {
                    grid[y][x] = false;
                    removed_rolls_of_paper += 1;
                    is_unchanged = false;
                }
            }
        }
    }

    Solution::U16(removed_rolls_of_paper)
}

fn get_grid(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|character| character == '@').collect())
        .collect()
}

fn is_roll_of_paper_accessible(grid: &[Vec<bool>], x: usize, y: usize) -> bool {
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
                    return false;
                }
            }
        }
    }

    true
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

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
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
            Solution::U8(43)
        );
    }
}
