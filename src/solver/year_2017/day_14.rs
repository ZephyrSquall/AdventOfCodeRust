use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2017,
    day: 14,
    title: "Disk Defragmentation",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let knot_hash_binaries = get_knot_hash_binaries(input);

    let mut used_squares = 0;

    for knot_hash_binary in knot_hash_binaries {
        used_squares += knot_hash_binary.count_ones();
    }

    Solution::U32(used_squares)
}

fn solve_2(input: &str) -> Solution {
    let knot_hash_binaries = get_knot_hash_binaries(input);

    // Primitive integers cannot be indexed to access their bits, so convert them to a vector of
    // booleans that's true for digits that are 1 (representing used squares) and false for digits
    // that are 0 (representing free squares).
    let mut knot_hash_bool_array = Vec::with_capacity(128);

    // Use bitwise operations to extract the individual bits. This utilizes the technique that
    // performing a bitwise AND operation with the number 1 zeroes out all digits except for the
    // least significant digit, which can be tested to determine its value. Then a right shift is
    // performed to move the next digit into the least significant bit position and the process
    // repeats for all 128 binary digits. Note that assigning the booleans this way flips the grid
    // horizontally, but that's fine because this doesn't affect the connectivity between adjacent
    // squares.
    for mut knot_hash_binary in knot_hash_binaries {
        let mut knot_hash_bool_row = Vec::with_capacity(128);
        for _ in 0..128 {
            let lsb = knot_hash_binary & 1;
            knot_hash_bool_row.push(lsb == 1);
            knot_hash_binary >>= 1;
        }
        knot_hash_bool_array.push(knot_hash_bool_row);
    }

    let mut groups = 0;
    let mut visited_points = Vec::with_capacity(128 * 128);
    for (y, knot_hash_bool_row) in knot_hash_bool_array.iter().enumerate() {
        for (x, square) in knot_hash_bool_row.iter().enumerate() {
            // Check if the current square is used and is not part of any previously-found group.
            if *square && !visited_points.contains(&Point { x, y }) {
                // Increment groups, then find all squares connected to this square and record them
                // so they aren't double-counted.
                groups += 1;
                let mut connected_squares =
                    get_connected_squares(Point { x, y }, &knot_hash_bool_array);
                visited_points.append(&mut connected_squares);
            }
        }
    }

    Solution::U32(groups)
}

// Take an input string, leverage Day 10 Part 2's solution to calculate its knot hash, then convert
// it from a hexadecimal string back to a numerical value. This value is exactly 128 bits long so
// Rust's u128 type is used.
fn get_knot_hash_binaries(input: &str) -> Vec<u128> {
    let mut knot_hash_binaries = Vec::with_capacity(128);

    for row in 0..128 {
        let hash_input = format!("{input}-{row}");
        let knot_hash = super::day_10::SOLVER.part_solvers[1](&hash_input).to_string();
        let knot_hash_binary = u128::from_str_radix(&knot_hash, 16).expect("Error parsing hash");

        knot_hash_binaries.push(knot_hash_binary);
    }

    knot_hash_binaries
}

#[derive(PartialEq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn get_connected_squares(starting_point: Point, grid: &Vec<Vec<bool>>) -> Vec<Point> {
    // This function will only be called on a used square, so no need to check if the starting point
    // is used.
    let mut connected_squares = vec![starting_point];

    // Recursively check all adjacent squares. Whenever another connected square is found,
    // connected_squares is mutated to include it, and the other three adjacent squares (immediately
    // moving backwards is avoided) are checked.
    check_up(starting_point, &mut connected_squares, grid);
    check_right(starting_point, &mut connected_squares, grid);
    check_down(starting_point, &mut connected_squares, grid);
    check_left(starting_point, &mut connected_squares, grid);

    connected_squares
}

fn check_up(mut point: Point, connected_squares: &mut Vec<Point>, grid: &Vec<Vec<bool>>) {
    // Make sure the current position is not on the top boundary. If so, there are no further
    // squares beyond the grid, so do nothing.
    if point.y < 127 {
        point.y += 1;
        // Check if the current square is a used square that hasn't already been found.
        if grid[point.y][point.x] && !connected_squares.contains(&point) {
            connected_squares.push(point);

            check_up(point, connected_squares, grid);
            check_right(point, connected_squares, grid);
            check_left(point, connected_squares, grid);
        }
    }
}

fn check_right(mut point: Point, connected_squares: &mut Vec<Point>, grid: &Vec<Vec<bool>>) {
    if point.x < 127 {
        point.x += 1;
        if grid[point.y][point.x] && !connected_squares.contains(&point) {
            connected_squares.push(point);

            check_up(point, connected_squares, grid);
            check_right(point, connected_squares, grid);
            check_down(point, connected_squares, grid);
        }
    }
}

fn check_down(mut point: Point, connected_squares: &mut Vec<Point>, grid: &Vec<Vec<bool>>) {
    if point.y > 0 {
        point.y -= 1;
        if grid[point.y][point.x] && !connected_squares.contains(&point) {
            connected_squares.push(point);

            check_right(point, connected_squares, grid);
            check_down(point, connected_squares, grid);
            check_left(point, connected_squares, grid);
        }
    }
}

fn check_left(mut point: Point, connected_squares: &mut Vec<Point>, grid: &Vec<Vec<bool>>) {
    if point.x > 0 {
        point.x -= 1;
        if grid[point.y][point.x] && !connected_squares.contains(&point) {
            connected_squares.push(point);

            check_up(point, connected_squares, grid);
            check_down(point, connected_squares, grid);
            check_left(point, connected_squares, grid);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("flqrgnkx"), Solution::U16(8108));
    }

    #[test]
    fn example2_1() {
        assert_eq!(solve_2("flqrgnkx"), Solution::U16(1242));
    }
}
