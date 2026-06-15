use crate::solver::{AdventOfCode, Solution};
use std::collections::VecDeque;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2023,
    day: 3,
    title: "Gear Ratios",
    part_solvers: &[solve_1, solve_2],
};

fn part_number_from_digits<'a>(digits: impl Iterator<Item = &'a char>) -> u32 {
    digits
        .map(|char| char.to_digit(10).expect("digit should be a valid number"))
        .reduce(|acc, digit| acc * 10 + digit)
        .expect("digits array should not be empty")
}

fn solve_1(input: &str) -> Solution {
    fn is_symbol(c: char) -> bool {
        !(c.is_ascii_digit() || c == '.')
    }

    fn search_for_adjacent_symbol(
        grid: &[Vec<char>],
        x: usize,
        y: usize,
        forwards_only: bool,
    ) -> bool {
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
                        part_number_sum += part_number_from_digits(previous_digits.iter());
                    }
                    is_adjacent_to_symbol = false;
                    previous_digits.clear();
                }
            }
        }
        // A separate check needs to be made at the end of a line, otherwise valid numbers directly
        // against the right edge of the grid would be missed.
        if !previous_digits.is_empty() && is_adjacent_to_symbol {
            part_number_sum += part_number_from_digits(previous_digits.iter());
        }
    }

    Solution::U32(part_number_sum)
}

fn solve_2(input: &str) -> Solution {
    fn get_gear_ratio(grid: &[Vec<char>], x: usize, y: usize) -> u32 {
        let mut adjacent_part_numbers = Vec::with_capacity(2);
        // There are three adjacent positions above the '*'. If the middle position (the one
        // directly above, not either of the ones diagonally above) is a digit, then any digits in
        // these three positions must belong to the same number. Otherwise, if there are digits in
        // both diagonal directions but not directly above, then these digits must belong to
        // separate numbers. This same logic applies to the three adjacent positions below the '*'.
        // No such check needs to be made for digits directly to the left and right, as the '*'
        // itself always separates these numbers.

        // Check north
        if let Some(y_sub) = y.checked_sub(1) {
            if grid[y_sub][x].is_ascii_digit() {
                adjacent_part_numbers.push(get_part_number_from_digit_position(grid, x, y_sub));
            } else {
                // Check north-west, only if check north failed
                if let Some(x_sub) = x.checked_sub(1)
                    && grid[y_sub][x_sub].is_ascii_digit()
                {
                    adjacent_part_numbers
                        .push(get_part_number_from_digit_position(grid, x_sub, y_sub));
                }

                // Check north-east, only if check north failed
                if let Some(value) = grid[y_sub].get(x + 1)
                    && value.is_ascii_digit()
                {
                    adjacent_part_numbers.push(get_part_number_from_digit_position(
                        grid,
                        x + 1,
                        y_sub,
                    ));
                }
            }
        }

        // Check south
        if let Some(grid_line) = grid.get(y + 1) {
            if grid_line[x].is_ascii_digit() {
                // Starting from south checks, it's possible three or more adjacent numbers have
                // been found, so start checking and return early if this occurred.
                if adjacent_part_numbers.len() == 2 {
                    return 0;
                }
                adjacent_part_numbers.push(get_part_number_from_digit_position(grid, x, y + 1));
            } else {
                // Check south-west, only if check north failed
                if let Some(x_sub) = x.checked_sub(1)
                    && grid_line[x_sub].is_ascii_digit()
                {
                    if adjacent_part_numbers.len() == 2 {
                        return 0;
                    }
                    adjacent_part_numbers.push(get_part_number_from_digit_position(
                        grid,
                        x_sub,
                        y + 1,
                    ));
                }

                // Check south-east, only if check north failed
                if let Some(value) = grid_line.get(x + 1)
                    && value.is_ascii_digit()
                {
                    if adjacent_part_numbers.len() == 2 {
                        return 0;
                    }
                    adjacent_part_numbers.push(get_part_number_from_digit_position(
                        grid,
                        x + 1,
                        y + 1,
                    ));
                }
            }
        }

        // Check east
        if let Some(value) = grid[y].get(x + 1)
            && value.is_ascii_digit()
        {
            if adjacent_part_numbers.len() == 2 {
                return 0;
            }
            adjacent_part_numbers.push(get_part_number_from_digit_position(grid, x + 1, y));
        }

        // Check west
        if let Some(x_sub) = x.checked_sub(1)
            && grid[y][x_sub].is_ascii_digit()
        {
            if adjacent_part_numbers.len() == 2 {
                return 0;
            }
            adjacent_part_numbers.push(get_part_number_from_digit_position(grid, x_sub, y));
        }

        if adjacent_part_numbers.len() == 2 {
            adjacent_part_numbers[0] * adjacent_part_numbers[1]
        } else {
            0
        }
    }

    // Given the position of a digit in the grid, search east and west from that position for more
    // digits and return the full number.
    fn get_part_number_from_digit_position(grid: &[Vec<char>], mut x: usize, y: usize) -> u32 {
        let mut digits = VecDeque::new();
        digits.push_back(grid[y][x]);

        // Search west
        let mut x_offset = x;
        while let Some(x_offset_valid) = x_offset.checked_sub(1) {
            x_offset = x_offset_valid;
            let value = grid[y][x_offset];

            if value.is_ascii_digit() {
                digits.push_front(value);
            } else {
                break;
            }
        }

        // Search east
        x += 1;
        while let Some(value) = grid[y].get(x) {
            if value.is_ascii_digit() {
                digits.push_back(*value);
            } else {
                break;
            }

            x += 1;
        }

        part_number_from_digits(digits.iter())
    }

    let mut gear_ratio_sum = 0;
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '*' {
                gear_ratio_sum += get_gear_ratio(&grid, x, y);
            }
        }
    }

    Solution::U32(gear_ratio_sum)
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

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
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
            Solution::U32(467_835)
        );
    }
}
