use crate::solver::{AdventOfCode, Solution};
use rustc_hash::FxHashMap;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2017,
    day: 21,
    title: "Fractal Art",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    solve(input, 5)
}

fn solve_2(input: &str) -> Solution {
    solve(input, 18)
}

// Convert a 2D grid of bools to its string representation in the list of enhancement rules.
fn serialize(grid: &[Vec<bool>]) -> String {
    let mut pattern = String::new();

    let mut iter = grid.iter().peekable();
    while let Some(grid_line) = iter.next() {
        for square in grid_line {
            if *square {
                pattern += "#";
            } else {
                pattern += ".";
            }
        }
        // Add a forward slash only if there is another grid row after this one.
        if iter.peek().is_some() {
            pattern += "/";
        }
    }

    pattern
}

// Convert a string representation in the list of enhancement rules to its corresponding 2D grid of
// bools.
fn deserialize(pattern: &str) -> Vec<Vec<bool>> {
    let mut grid = Vec::new();

    for line in pattern.split('/') {
        let mut grid_line = Vec::new();
        for character in line.chars() {
            if character == '#' {
                grid_line.push(true);
            } else {
                grid_line.push(false);
            }
        }
        grid.push(grid_line);
    }

    grid
}

// Take an enhancement rule and generate all 7 variations of it that can be created by a combination
// of rotations and flips. Note that duplicate patterns may be returned.
fn get_all_rotations_and_flips(pattern: &str) -> [String; 7] {
    let original_grid = deserialize(pattern);
    let grid_size = original_grid.len();

    let mut blank_grid = Vec::new();
    for _ in 0..grid_size {
        blank_grid.push(vec![false; grid_size]);
    }

    let mut rotate_90 = blank_grid.clone();
    let mut rotate_180 = blank_grid.clone();
    let mut rotate_270 = blank_grid.clone();
    let mut original_flip = blank_grid.clone();
    let mut rotate_90_flip = blank_grid.clone();
    let mut rotate_180_flip = blank_grid.clone();
    let mut rotate_270_flip = blank_grid;

    for x in 0..grid_size {
        for y in 0..grid_size {
            if original_grid[y][x] {
                rotate_90[x][grid_size - y - 1] = true;
                rotate_180[grid_size - y - 1][grid_size - x - 1] = true;
                rotate_270[grid_size - x - 1][y] = true;
                original_flip[y][grid_size - x - 1] = true;
                rotate_90_flip[grid_size - x - 1][grid_size - y - 1] = true;
                rotate_180_flip[grid_size - y - 1][x] = true;
                rotate_270_flip[x][y] = true;
            }
        }
    }

    [
        serialize(&rotate_90),
        serialize(&rotate_180),
        serialize(&rotate_270),
        serialize(&original_flip),
        serialize(&rotate_90_flip),
        serialize(&rotate_180_flip),
        serialize(&rotate_270_flip),
    ]
}

// Take a 2D grid of bools, and divide it up to create a 2D grid of 2D grids of bools. In this
// context, the inner grids are called the "base grid" and the outer grid is called the "divided
// grid". If the original grid's size is divisible by 2, then the base grids will have size 2,
// otherwise the base grids will have size 3.
fn get_divided_grid(grid: &[Vec<bool>]) -> Vec<Vec<Vec<Vec<bool>>>> {
    let base_grid_size = if grid.len().is_multiple_of(2) { 2 } else { 3 };
    let mut divided_grid = Vec::new();

    // Iterate over 2 or 3 lines at a time (depending on base_grid_size).
    for grouped_grid_lines in grid.chunks(base_grid_size) {
        // Set up a vector which contains (original grid size / base grid size) inner vectors. Each
        // of these inner vectors represents a base grid.
        let mut divided_grid_line = vec![Vec::new(); grid.len() / base_grid_size];
        for grid_line in grouped_grid_lines {
            // Get each base grid, and use the index to get the appropriate inner vector in
            // divided_grid_lines to push all of the base grid's lines to.
            for (index, base_grid_line) in grid_line.chunks(base_grid_size).enumerate() {
                let base_grid_line = Vec::from(base_grid_line);
                divided_grid_line[index].push(base_grid_line);
            }
        }
        divided_grid.push(divided_grid_line);
    }

    divided_grid
}

// Take a 2D grid of 2D grid of bools, and reconnect everything into a single 2D grid of bools.
fn get_reconnected_grid(divided_grid: &Vec<Vec<Vec<Vec<bool>>>>) -> Vec<Vec<bool>> {
    // Use the first base grid to determine the base grid size (it is a logic error to call this
    // function with an empty grid).
    let base_grid_size = divided_grid[0][0].len();
    let mut grid = Vec::new();

    for divided_grid_line in divided_grid {
        // Each divided grid line expands to 2 or 3 lines in the reconnected grid (depending on
        // base_grid_size). Set up a vector which contains this number of empty lines.
        let mut grid_lines = vec![Vec::new(); base_grid_size];
        for base_grid in divided_grid_line {
            // Get each line in the base grid, and use the index to get the appropriate line of the
            // reconnected grid to push each base grid line to. Each base grid in a single row of a
            // divided_grid_line is iterated in order, so the first line of every base grid is added
            // to the first line of the reconnected grid in order, and so on for the rest of the
            // base grid's lines.
            for (index, base_grid_line) in base_grid.iter().enumerate() {
                grid_lines[index].extend_from_slice(base_grid_line);
            }
        }
        grid.append(&mut grid_lines);
    }

    grid
}

fn solve(input: &str, iterations: u8) -> Solution {
    // Construct a map containing all enhancement rules. For every enhancement rule in the puzzle
    // input, add all 8 variants of its input (all 4 rotations, plus all 4 rotations again but
    // flipped) so no further rotation and flipping is required to find matches. This also
    // eliminates duplication as any symmetric pattern that becomes itself when rotated and/or
    // flipped will overwrite its key in the hashmap, only leaving unique results.
    let mut enhancement_rules = FxHashMap::default();
    for line in input.lines() {
        let mut iter = line.split(" => ");
        let input_pattern = iter.next().expect("Enhancement rule should have an input");
        let output_pattern = iter.next().expect("Enhancement rule should have an output");

        // Add the enhancement rule with its original input.
        enhancement_rules.insert(input_pattern.to_string(), output_pattern);

        // Add the enhancement rule 7 more times, once for every possible variation on its input.
        let rotated_and_flipped_patterns = get_all_rotations_and_flips(input_pattern);
        for rotated_and_flipped_pattern in rotated_and_flipped_patterns {
            enhancement_rules.insert(rotated_and_flipped_pattern.to_string(), output_pattern);
        }
    }

    // Starting grid.
    let mut grid = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];

    for _ in 0..iterations {
        // Divide the grid.
        let divided_grid = get_divided_grid(&grid);
        // Create a new divided grid to hold the output of this iteration.
        let mut new_divided_grid = Vec::new();

        // For each base grid, apply the corresponding enhancement rule and place the output in the
        // new divided grid.
        for divided_grid_line in divided_grid {
            let mut new_divided_grid_line = Vec::new();
            for base_grid in divided_grid_line {
                let pattern = serialize(&base_grid);
                let new_pattern = enhancement_rules
                    .get(&pattern)
                    .expect("All patterns should have corresponding enhancement rule");
                let new_base_grid = deserialize(new_pattern);
                new_divided_grid_line.push(new_base_grid);
            }
            new_divided_grid.push(new_divided_grid_line);
        }

        // Reconnect the new divided grid and overwrite the starting grid with it so it becomes the
        // starting point for the next iteration.
        grid = get_reconnected_grid(&new_divided_grid);
    }

    let mut pixel_count = 0;
    for grid_line in grid {
        for square in grid_line {
            if square {
                pixel_count += 1;
            }
        }
    }
    Solution::U32(pixel_count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve(
                "\
../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#",
                2
            ),
            Solution::U8(12)
        );
    }
}
