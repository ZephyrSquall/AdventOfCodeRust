use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2017,
    day: 19,
    title: "A Series of Tubes",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    // Pushes the letter in the grid at the given position into visited_letters, unless it's a
    // regular path character ('|', '-', or '+').
    fn update_visited_letters(
        grid: &[Vec<char>],
        visited_letters: &mut Vec<char>,
        x: usize,
        y: usize,
    ) {
        let letter = grid[y][x];
        if letter != '|' && letter != '-' && letter != '+' {
            visited_letters.push(letter);
        }
    }

    let mut visited_letters = Vec::new();
    traverse_path(input, &mut update_visited_letters, &mut visited_letters);
    // Convert array of chars into String.
    let visited_letters = visited_letters.into_iter().collect();
    Solution::String(visited_letters)
}

fn solve_2(input: &str) -> Solution {
    fn update_count(_: &[Vec<char>], count: &mut u32, _: usize, _: usize) {
        *count += 1;
    }

    // Start count from 1 to include the step of entering the grid from outside the top of the grid.
    let mut count = 1;
    traverse_path(input, &mut update_count, &mut count);
    Solution::U32(count)
}

// Builds the grid using the input, identifies the starting position, then traverses the path until
// it terminates. On each step, calls the provided update function with the provided update value
// reference to calculate whatever answer is required.
fn traverse_path<T, F: FnMut(&[Vec<char>], &mut T, usize, usize)>(
    input: &str,
    update: &mut F,
    update_value: &mut T,
) {
    enum Direction {
        Up,
        Right,
        Down,
        Left,
    }

    let mut grid = Vec::new();
    for line in input.lines() {
        let mut grid_line = Vec::new();
        for char in line.chars() {
            grid_line.push(char);
        }
        grid.push(grid_line);
    }

    // Get the packet's starting position and direction, which is downwards from where the only
    // non-space character is located in the first row.
    let mut x = grid[0]
        .iter()
        .position(|c| *c != ' ')
        .expect("Should have a non-space character in first row");
    let mut y = 0;
    let mut direction = Direction::Down;

    loop {
        match direction {
            Direction::Up => {
                y -= 1;
                update(&grid, update_value, x, y);

                if grid[y - 1][x] == ' ' {
                    if grid[y][x + 1] != ' ' {
                        direction = Direction::Right;
                    } else if grid[y][x - 1] != ' ' {
                        direction = Direction::Left;
                    } else {
                        break;
                    }
                }
            }
            Direction::Right => {
                x += 1;
                update(&grid, update_value, x, y);

                if grid[y][x + 1] == ' ' {
                    if grid[y + 1][x] != ' ' {
                        direction = Direction::Down;
                    } else if grid[y - 1][x] != ' ' {
                        direction = Direction::Up;
                    } else {
                        break;
                    }
                }
            }
            Direction::Down => {
                y += 1;
                update(&grid, update_value, x, y);

                if grid[y + 1][x] == ' ' {
                    if grid[y][x + 1] != ' ' {
                        direction = Direction::Right;
                    } else if grid[y][x - 1] != ' ' {
                        direction = Direction::Left;
                    } else {
                        break;
                    }
                }
            }
            Direction::Left => {
                x -= 1;
                update(&grid, update_value, x, y);

                if grid[y][x - 1] == ' ' {
                    if grid[y + 1][x] != ' ' {
                        direction = Direction::Down;
                    } else if grid[y - 1][x] != ' ' {
                        direction = Direction::Up;
                    } else {
                        break;
                    }
                }
            }
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
                // In Rust, an end-of-line backslash ignores the newline and all whitespace at the
                // beginning of the following line. In this case, that whitespace is intended to be
                // part of the string, so a backslash can't be used to align this string literal.
                "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
                "
            ),
            Solution::Str("ABCDEF")
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
                "
            ),
            Solution::U8(38)
        );
    }
}
