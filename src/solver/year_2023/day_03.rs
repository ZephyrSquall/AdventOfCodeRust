use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2023,
    day: 3,
    title: "Gear Ratios",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let mut part_number_sum = 0;
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for y in 0..grid.len() {
        let mut is_adjacent_to_symbol = false;
        let mut previous_digits = Vec::new();

        for x in 0..grid[y].len() {
            let current_value = grid[y][x];
            if current_value.is_ascii_digit() {
                // If on a digit and no symbol has previously been found, look for one.
                if !is_adjacent_to_symbol {
                    is_adjacent_to_symbol =
                        search_for_adjacent_symbol(&grid, x, y, !previous_digits.is_empty());
                }
                previous_digits.push(current_value);
            } else {
                if !previous_digits.is_empty() {
                    if is_adjacent_to_symbol || is_symbol(current_value) {
                        part_number_sum += part_number_from_digits(&previous_digits);
                    }
                    is_adjacent_to_symbol = false;
                    previous_digits.clear();
                }
            }
        }
        // A separate check needs to be made at the end of a line, otherwise valid numbers directly
        // against the right edge of the grid would be missed.
        if !previous_digits.is_empty() && is_adjacent_to_symbol {
            part_number_sum += part_number_from_digits(&previous_digits);
        }
    }

    Solution::U32(part_number_sum)
}

fn is_symbol(c: char) -> bool {
    !(c.is_ascii_digit() || c == '.')
}

fn search_for_adjacent_symbol(grid: &[Vec<char>], x: usize, y: usize, forwards_only: bool) -> bool {
    // Search north-east
    if let Some(y) = y.checked_sub(1)
        && let Some(char) = grid[y].get(x + 1)
        && is_symbol(*char)
    {
        return true;
    }
    // Search south-east
    if let Some(grid_line) = grid.get(y + 1)
        && let Some(char) = grid_line.get(x + 1)
        && is_symbol(*char)
    {
        return true;
    }

    if !forwards_only {
        // Search north
        if let Some(y) = y.checked_sub(1)
            && is_symbol(grid[y][x])
        {
            return true;
        }
        // Search north-west
        if let Some(y) = y.checked_sub(1)
            && let Some(x) = x.checked_sub(1)
            && is_symbol(grid[y][x])
        {
            return true;
        }
        // Search west
        if let Some(x) = x.checked_sub(1)
            && is_symbol(grid[y][x])
        {
            return true;
        }
        // Search south-west
        if let Some(x) = x.checked_sub(1)
            && let Some(grid_line) = grid.get(y + 1)
            && is_symbol(grid_line[x])
        {
            return true;
        }
        // Search south
        if let Some(grid_line) = grid.get(y + 1)
            && is_symbol(grid_line[x])
        {
            return true;
        }
    }

    false
}

fn part_number_from_digits(digits: &[char]) -> u32 {
    digits
        .iter()
        .map(|char| char.to_digit(10).expect("digit should be a valid number"))
        .reduce(|acc, digit| acc * 10 + digit)
        .expect("digits array should not be empty")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            Solution::U16(4361)
        );
    }
}
