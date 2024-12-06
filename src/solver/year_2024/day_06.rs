use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 6,
    title: "Guard Gallivant",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    struct Position {
        x: usize,
        y: usize,
    }

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

            // Attempt to add dx to x and dy to y. If it overflows (goes below 0), skip to the end
            // and return None.
            if let Some((x_next, y_next)) = position
                .x
                .checked_add_signed(dx)
                .zip(position.y.checked_add_signed(dy))
            {
                // Check if the new position is greater than the lengths of the map array. If so,
                // skip to the end and return None.
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

    // map_obstacles is a 2D bool array of all map squares. It's true for squares with obstacles
    // and false for open squares.
    let mut map_obstacles = Vec::new();
    // map_visited is a 2D bool array of all map squares. It's true for squares the guard has
    // visited at least once and false for squares the guard has never visited.
    let mut map_visited = Vec::new();
    let mut guard_position = Position { x: 0, y: 0 };
    let mut direction = Direction::Up;

    // Read the map to get all map obstacles and the guard's starting position. Also initialize
    // map_visited_ to be true on the guard's starting position and false everywhere else.
    for (y, line) in input.lines().enumerate() {
        let mut map_obstacles_line = Vec::new();
        let mut map_visited_line = Vec::new();
        for (x, character) in line.chars().enumerate() {
            match character {
                '.' => {
                    map_obstacles_line.push(false);
                    map_visited_line.push(false);
                }
                '#' => {
                    map_obstacles_line.push(true);
                    map_visited_line.push(false);
                }
                '^' => {
                    map_obstacles_line.push(false);
                    map_visited_line.push(true);
                    guard_position = Position { x, y };
                }
                _ => panic!("Unexpected character in map"),
            }
        }
        map_obstacles.push(map_obstacles_line);
        map_visited.push(map_visited_line);
    }

    let x_len = map_obstacles[0].len();
    let y_len = map_obstacles.len();

    // Continue moving to new squares until the guard leaves the map.
    while let Some(next_position) = direction.next_square(&guard_position, x_len, y_len) {
        if map_obstacles[next_position.y][next_position.x] {
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
}
