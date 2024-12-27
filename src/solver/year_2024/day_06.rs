use crate::solver::{Solution, AdventOfCode};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2024,
    day: 6,
    title: "Guard Gallivant",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let (map_obstructions, mut map_visited, mut guard_position) = get_map(input);
    let mut direction = Direction::Up;
    let x_len = map_obstructions[0].len();
    let y_len = map_obstructions.len();

    // Continue moving to new squares until the guard leaves the map.
    while let Some(next_position) = direction.next_square(&guard_position, x_len, y_len) {
        if map_obstructions[next_position.y][next_position.x] {
            direction.rotate();
        } else {
            guard_position.x = next_position.x;
            guard_position.y = next_position.y;
            map_visited[next_position.y][next_position.x] = true;
        }
    }

    // Count the visited squares.
    let mut visited_squares = 0;
    for map_visited_line in map_visited {
        for map_visited_square in map_visited_line {
            if map_visited_square {
                visited_squares += 1;
            }
        }
    }

    Solution::U32(visited_squares)
}

fn solve_2(input: &str) -> Solution {
    // Checks if placing an obstruction on the square after the current square would cause the guard
    // to go into a loop.
    fn would_new_obstruction_create_loop(
        map_obstructions: &[Vec<bool>],
        mut guard_position: Position,
        mut direction: Direction,
        x_len: usize,
        y_len: usize,
        new_obstruction_position: &Position,
    ) -> bool {
        // It's not enough for a guard to merely revisit a previous square. For a loop to form, the
        // guard has to revisit a previous square with the same direction as before. This can be
        // ensured by noting the starting direction and only recording squares visited while the
        // guard is moving in that direction. Note that since the new obstruction is directly in
        // front of the guard, they will always begin with a rotation, so set the starting direction
        // after this initial rotation.
        direction.rotate();
        let starting_direction = direction.clone();

        let mut map_visited_in_starting_direction = vec![vec![false; x_len]; y_len];
        map_visited_in_starting_direction[guard_position.y][guard_position.x] = true;

        while let Some(next_position) = direction.next_square(&guard_position, x_len, y_len) {
            if map_obstructions[next_position.y][next_position.x]
                || (next_position.x == new_obstruction_position.x
                    && next_position.y == new_obstruction_position.y)
            {
                direction.rotate();

                // It is important to check if the guard is facing the right direction after
                // rotating too, as there are some loops that go back and forth along a single line
                // due to both ends of the line having two obstructions placed such that the guard
                // rotates twice in a row. Without this check, such loops would be missed if the
                // line direction is different to the starting direction.
                if direction == starting_direction {
                    if map_visited_in_starting_direction[guard_position.y][guard_position.x] {
                        return true;
                    }
                    map_visited_in_starting_direction[guard_position.y][guard_position.x] = true;
                }
            } else {
                guard_position.x = next_position.x;
                guard_position.y = next_position.y;
                if direction == starting_direction {
                    if map_visited_in_starting_direction[next_position.y][next_position.x] {
                        return true;
                    }
                    map_visited_in_starting_direction[next_position.y][next_position.x] = true;
                }
            }
        }

        false
    }

    let (map_obstructions, mut map_visited, mut guard_position) = get_map(input);
    let mut direction = Direction::Up;
    let x_len = map_obstructions[0].len();
    let y_len = map_obstructions.len();

    let mut new_obstruction_positions = 0;

    // Continue moving to new squares until the guard leaves the map.
    while let Some(next_position) = direction.next_square(&guard_position, x_len, y_len) {
        if map_obstructions[next_position.y][next_position.x] {
            direction.rotate();
        } else {
            // Only check to see if placing an obstruction would cause a loop if the next square
            // hasn't been visited before. If it had been visited before, then it has already been
            // considered for having an obstruction placed there, and the guard would not have
            // followed the path they just did if that obstruction was there to deflect them the
            // first time they reached that position.
            if !map_visited[next_position.y][next_position.x]
                && would_new_obstruction_create_loop(
                    &map_obstructions,
                    guard_position.clone(),
                    direction.clone(),
                    x_len,
                    y_len,
                    &Position {
                        x: next_position.x,
                        y: next_position.y,
                    },
                )
            {
                new_obstruction_positions += 1;
            }
            guard_position.x = next_position.x;
            guard_position.y = next_position.y;
            map_visited[next_position.y][next_position.x] = true;
        }
    }

    Solution::U32(new_obstruction_positions)
}

#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    // Find the next square from the given position in the current direction. If this would go
    // outside the bounds of the map, returns None.
    fn next_square(&self, position: &Position, x_len: usize, y_len: usize) -> Option<Position> {
        let dx;
        let dy;
        match self {
            Direction::Up => {
                dx = 0;
                dy = -1;
            }
            Direction::Right => {
                dx = 1;
                dy = 0;
            }
            Direction::Down => {
                dx = 0;
                dy = 1;
            }
            Direction::Left => {
                dx = -1;
                dy = 0;
            }
        }

        // Attempt to add dx to x and dy to y. If it overflows (goes below 0), skip to the end and
        // return None.
        if let Some((x_next, y_next)) = position
            .x
            .checked_add_signed(dx)
            .zip(position.y.checked_add_signed(dy))
        {
            // Check if the new position is greater than the lengths of the map array. If so, skip
            // to the end and return None.
            if x_next < x_len && y_next < y_len {
                // The next position is still in the map bounds, so return the next position.
                return Some(Position {
                    x: x_next,
                    y: y_next,
                });
            }
        }
        None
    }

    // Rotate 90 degrees to the right.
    fn rotate(&mut self) {
        match self {
            Direction::Up => {
                *self = Direction::Right;
            }
            Direction::Right => {
                *self = Direction::Down;
            }
            Direction::Down => {
                *self = Direction::Left;
            }
            Direction::Left => {
                *self = Direction::Up;
            }
        }
    }
}

// Read the map to get all map obstructions and the guard's starting position. Also initialize
// map_visited_ to be true on the guard's starting position and false everywhere else.
fn get_map(input: &str) -> (Vec<Vec<bool>>, Vec<Vec<bool>>, Position) {
    // map_obstructions is a 2D bool array of all map squares. It's true for squares with obstructions and
    // false for open squares.
    let mut map_obstructions = Vec::new();
    // map_visited is a 2D bool array of all map squares. It's true for squares the guard has
    // visited at least once and false for squares the guard has never visited.
    let mut map_visited = Vec::new();
    let mut guard_position = Position { x: 0, y: 0 };

    for (y, line) in input.lines().enumerate() {
        let mut map_obstructions_line = Vec::new();
        let mut map_visited_line = Vec::new();
        for (x, character) in line.chars().enumerate() {
            match character {
                '.' => {
                    map_obstructions_line.push(false);
                    map_visited_line.push(false);
                }
                '#' => {
                    map_obstructions_line.push(true);
                    map_visited_line.push(false);
                }
                '^' => {
                    map_obstructions_line.push(false);
                    map_visited_line.push(true);
                    guard_position = Position { x, y };
                }
                _ => panic!("Unexpected character in map"),
            }
        }
        map_obstructions.push(map_obstructions_line);
        map_visited.push(map_visited_line);
    }

    (map_obstructions, map_visited, guard_position)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            ),
            Solution::U8(41)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            ),
            Solution::U8(6)
        );
    }
}
