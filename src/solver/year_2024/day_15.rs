use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 15,
    title: "Warehouse Woes",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    #[derive(PartialEq)]
    enum Tile {
        Wall,
        Empty,
        Box,
    }

    #[derive(Clone)]
    struct Position {
        x: usize,
        y: usize,
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
}
