use crate::solver::{AdventOfCode, Solution};
use rustc_hash::FxHashSet;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2024,
    day: 15,
    title: "Warehouse Woes",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    #[derive(PartialEq)]
    enum Tile {
        Wall,
        Empty,
        Box,
    }

    let mut warehouse = Vec::new();
    let mut robot_position = Position { x: 0, y: 0 };

    // Get the map.
    let mut line_iter = input.lines();
    // Must take line_iter by ref so it can continue to be iterated after this for loop.
    for (y, line) in line_iter.by_ref().enumerate() {
        // If the line is empty, this is the empty line between the map and the directions, so move
        // on to executing the directions.
        if line.is_empty() {
            break;
        }

        let mut warehouse_line = Vec::new();
        for (x, tile) in line.chars().enumerate() {
            match tile {
                '#' => warehouse_line.push(Tile::Wall),
                '.' => warehouse_line.push(Tile::Empty),
                'O' => warehouse_line.push(Tile::Box),
                '@' => {
                    robot_position.x = x;
                    robot_position.y = y;
                    warehouse_line.push(Tile::Empty);
                }
                _ => panic!("Warehouse map has unexpected character"),
            }
        }
        warehouse.push(warehouse_line);
    }

    // Move the robot according to the direction sequence. Iterating over lines removes all newline
    // characters in the sequence, so they don't have to be handled.
    for line in line_iter {
        for direction in line.chars() {
            let dx;
            let dy;
            match direction {
                '^' => {
                    dx = 0;
                    dy = -1;
                }
                '>' => {
                    dx = 1;
                    dy = 0;
                }
                'v' => {
                    dx = 0;
                    dy = 1;
                }
                '<' => {
                    dx = -1;
                    dy = 0;
                }
                _ => panic!("Directions has unexpected character"),
            }

            let next_robot_position = Position {
                // robot_position.x and robot_position.y are both at least 1 due to the map being
                // surrounded by walls, and dx and dy are both at least -1, so there's no way to
                // underflow by adding them. Using a wrapping add as it is the quickest add (outside
                // of unsafe code) when overflow checks aren't required.
                x: robot_position.x.wrapping_add_signed(dx),
                y: robot_position.y.wrapping_add_signed(dy),
            };

            let can_robot_move;
            // Due to the buffer the wall provides, the next robot position will always have a valid
            // index.
            match warehouse[next_robot_position.y][next_robot_position.x] {
                Tile::Wall => can_robot_move = false,
                Tile::Empty => can_robot_move = true,
                Tile::Box => {
                    // Find the next position in that direction that doesn't have a box. Since a
                    // wall will always be encountered before the end of the map, there is also no
                    // chance of overflow here.
                    let mut next_box_position = Position {
                        x: next_robot_position.x.wrapping_add_signed(dx),
                        y: next_robot_position.y.wrapping_add_signed(dy),
                    };
                    while warehouse[next_box_position.y][next_box_position.x] == Tile::Box {
                        next_box_position.x = next_box_position.x.wrapping_add_signed(dx);
                        next_box_position.y = next_box_position.y.wrapping_add_signed(dy);
                    }

                    if warehouse[next_box_position.y][next_box_position.x] == Tile::Empty {
                        // To push a bunch of aligned boxes, simply takes the first box and move it
                        // to the end.
                        warehouse[next_box_position.y][next_box_position.x] = Tile::Box;
                        warehouse[next_robot_position.y][next_robot_position.x] = Tile::Empty;
                        can_robot_move = true;
                    } else {
                        can_robot_move = false;
                    }
                }
            }

            if can_robot_move {
                robot_position = next_robot_position;
            }
        }
    }

    // Get the GPS sum. Skip the first row and column since they contain nothing but walls, and
    // hence can't contain any boxes.
    let mut gps_sum = 0;
    for (y, warehouse_line) in warehouse.iter().enumerate().skip(1) {
        for (x, tile) in warehouse_line.iter().enumerate().skip(1) {
            if *tile == Tile::Box {
                gps_sum += x + 100 * y;
            }
        }
    }
    Solution::USize(gps_sum)
}

fn solve_2(input: &str) -> Solution {
    #[derive(PartialEq, Clone)]
    enum Tile {
        Wall,
        Empty,
        BoxLeft,
        BoxRight,
    }

    let mut warehouse = Vec::new();
    let mut robot_position = Position { x: 0, y: 0 };

    let mut line_iter = input.lines();
    for (y, line) in line_iter.by_ref().enumerate() {
        if line.is_empty() {
            break;
        }

        let mut warehouse_line = Vec::new();
        for (x, tile) in line.chars().enumerate() {
            match tile {
                '#' => {
                    warehouse_line.push(Tile::Wall);
                    warehouse_line.push(Tile::Wall);
                }
                '.' => {
                    warehouse_line.push(Tile::Empty);
                    warehouse_line.push(Tile::Empty);
                }
                'O' => {
                    warehouse_line.push(Tile::BoxLeft);
                    warehouse_line.push(Tile::BoxRight);
                }
                '@' => {
                    robot_position.x = x * 2;
                    robot_position.y = y;
                    warehouse_line.push(Tile::Empty);
                    warehouse_line.push(Tile::Empty);
                }
                _ => panic!("Warehouse map has unexpected character"),
            }
        }
        warehouse.push(warehouse_line);
    }

    for line in line_iter {
        for direction in line.chars() {
            let dx;
            let dy;
            match direction {
                '^' => {
                    dx = 0;
                    dy = -1;
                }
                '>' => {
                    dx = 1;
                    dy = 0;
                }
                'v' => {
                    dx = 0;
                    dy = 1;
                }
                '<' => {
                    dx = -1;
                    dy = 0;
                }
                _ => panic!("Directions has unexpected character"),
            }

            let next_robot_position = Position {
                x: robot_position.x.wrapping_add_signed(dx),
                y: robot_position.y.wrapping_add_signed(dy),
            };

            let can_robot_move;
            match warehouse[next_robot_position.y][next_robot_position.x] {
                Tile::Wall => can_robot_move = false,
                Tile::Empty => can_robot_move = true,
                Tile::BoxLeft | Tile::BoxRight => {
                    // If the movement is horizontal, check if the next non-box position in that
                    // direction is empty. If so, we can reuse the trick of moving the front box to
                    // the end, but then every tile in between needs to be switched to the other
                    // kind of box.
                    //
                    // This requires subtracting an isize from a usize, which requires casting, but
                    // the isize can only have values from -1 to 1 and the usize is always greater
                    // than 1 due to the wall buffer, so sign loss and wrapping can't happen.
                    #[allow(clippy::cast_sign_loss)]
                    #[allow(clippy::cast_possible_wrap)]
                    if dy == 0 {
                        // Keep searching left/right until something other than a box is found.
                        let mut next_box_position = Position {
                            x: next_robot_position.x.wrapping_add_signed(dx),
                            y: next_robot_position.y,
                        };
                        while warehouse[next_box_position.y][next_box_position.x] == Tile::BoxLeft
                            || warehouse[next_box_position.y][next_box_position.x] == Tile::BoxRight
                        {
                            next_box_position.x = next_box_position.x.wrapping_add_signed(dx);
                        }

                        // If it's an empty position, push the boxes into it.
                        if warehouse[next_box_position.y][next_box_position.x] == Tile::Empty {
                            // Take the box from the front and move it to the end.
                            warehouse[next_box_position.y][next_box_position.x] =
                                warehouse[next_robot_position.y][next_robot_position.x].clone();
                            warehouse[next_robot_position.y][next_robot_position.x] = Tile::Empty;

                            // Flip every box tile from a left to a right or vice versa.
                            while next_box_position.x != next_robot_position.x {
                                if warehouse[next_box_position.y][next_box_position.x]
                                    == Tile::BoxLeft
                                {
                                    warehouse[next_box_position.y][next_box_position.x] =
                                        Tile::BoxRight;
                                } else {
                                    warehouse[next_box_position.y][next_box_position.x] =
                                        Tile::BoxLeft;
                                }
                                next_box_position.x =
                                    ((next_box_position.x as isize) - dx) as usize;
                            }
                            can_robot_move = true;
                        } else {
                            can_robot_move = false;
                        }

                    // If the movement is vertical, identify the two box positions above/below the
                    // robot. For each position in that row, check the next position in that
                    // vertical direction. If it's a box, get the position of both box tiles in that
                    // box, and add it to a new list for the next row. Keep repeating this process
                    // for each row until every box in a row is checked and no new list is created.
                    // No new list means all the boxes have empty spaces above/below them, so move
                    // every box in every listed position in that vertical direction. If a wall is
                    // ever seen, it will block the boxes from being pushed, so cancel the entire
                    // movement.
                    } else {
                        // Set up the first row. Use a hashmap for each row to avoid double-counting
                        // box positions.
                        let mut box_rows = Vec::new();
                        let mut first_row_positions = FxHashSet::default();
                        // Get the half of the box at the next position.
                        first_row_positions.insert(next_robot_position.clone());
                        // Get the other half of that box.
                        if warehouse[next_robot_position.y][next_robot_position.x] == Tile::BoxLeft
                        {
                            first_row_positions.insert(Position {
                                x: next_robot_position.x + 1,
                                y: next_robot_position.y,
                            });
                        } else {
                            first_row_positions.insert(Position {
                                x: next_robot_position.x - 1,
                                y: next_robot_position.y,
                            });
                        }
                        box_rows.push(first_row_positions);

                        // Keep checking rows until either a wall is encountered or every next
                        // position is empty. box_row_index indicates the index of the previous row
                        // (to let its box positions be read) while y indicates the current row
                        // where new box positions above/below the previous row are being checked.
                        let mut box_row_index = 0;
                        let mut y = next_robot_position.y.wrapping_add_signed(dy);
                        let mut was_wall_found = false;
                        loop {
                            let mut next_row_positions = FxHashSet::default();
                            for box_position in &box_rows[box_row_index] {
                                if warehouse[y][box_position.x] == Tile::Wall {
                                    was_wall_found = true;
                                    break;
                                } else if warehouse[y][box_position.x] == Tile::BoxLeft {
                                    next_row_positions.insert(Position {
                                        x: box_position.x,
                                        y,
                                    });
                                    next_row_positions.insert(Position {
                                        x: box_position.x + 1,
                                        y,
                                    });
                                } else if warehouse[y][box_position.x] == Tile::BoxRight {
                                    next_row_positions.insert(Position {
                                        x: box_position.x,
                                        y,
                                    });
                                    next_row_positions.insert(Position {
                                        x: box_position.x - 1,
                                        y,
                                    });
                                }
                            }

                            if next_row_positions.is_empty() {
                                break;
                            }

                            box_rows.push(next_row_positions);
                            box_row_index += 1;
                            y = y.wrapping_add_signed(dy);
                        }

                        if was_wall_found {
                            can_robot_move = false;
                        } else {
                            // Push every box above/below the robot.
                            for box_row in box_rows.iter().rev() {
                                for box_position in box_row {
                                    warehouse[box_position.y.wrapping_add_signed(dy)]
                                        [box_position.x] =
                                        warehouse[box_position.y][box_position.x].clone();
                                    warehouse[box_position.y][box_position.x] = Tile::Empty;
                                }
                            }
                            can_robot_move = true;
                        }
                    }
                }
            }

            if can_robot_move {
                robot_position = next_robot_position;
            }
        }
    }

    // From the top-left corner, the left side of a box is always the closest side.
    let mut gps_sum = 0;
    for (y, warehouse_line) in warehouse.iter().enumerate().skip(1) {
        for (x, tile) in warehouse_line.iter().enumerate().skip(1) {
            if *tile == Tile::BoxLeft {
                gps_sum += x + 100 * y;
            }
        }
    }
    Solution::USize(gps_sum)
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            ),
            Solution::U16(10092)
        );
    }
    #[test]
    fn example1_2() {
        assert_eq!(
            solve_1(
                "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
            ),
            Solution::U16(2028)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            ),
            Solution::U16(9021)
        );
    }
}
