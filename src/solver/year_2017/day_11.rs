use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2017,
    day: 11,
    title: "Hex Ed",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    // Every space on the hex grid can be reached using only moves along the north-south axis and
    // the northeast-southwest axis (a southeast move is a northeast and south move together, and a
    // northwest move is a southwest and north move together). Let the northeast-southwest axis be
    // the x-axis and the north-south axis be the y-axis (northeast and north are the positive
    // directions).
    let mut hex_x: i32 = 0;
    let mut hex_y: i32 = 0;

    for direction in input.split(',') {
        step(direction, &mut hex_x, &mut hex_y);
    }

    Solution::I32(get_distance(hex_x, hex_y))
}

fn solve_2(input: &str) -> Solution {
    let mut hex_x: i32 = 0;
    let mut hex_y: i32 = 0;
    let mut furthest_distance = 0;

    for direction in input.split(',') {
        step(direction, &mut hex_x, &mut hex_y);

        let distance = get_distance(hex_x, hex_y);
        if distance > furthest_distance {
            furthest_distance = distance;
        }
    }

    Solution::I32(furthest_distance)
}

fn step(direction: &str, hex_x: &mut i32, hex_y: &mut i32) {
    match direction {
        "n" => {
            *hex_y += 1;
        }
        "ne" => {
            *hex_x += 1;
        }
        "se" => {
            *hex_x += 1;
            *hex_y -= 1;
        }
        "s" => {
            *hex_y -= 1;
        }
        "sw" => {
            *hex_x -= 1;
        }
        "nw" => {
            *hex_x -= 1;
            *hex_y += 1;
        }
        _ => {
            panic!("Invalid direction");
        }
    }
}

fn get_distance(mut hex_x: i32, mut hex_y: i32) -> i32 {
    // A southeast or northwest step shifts along the x- and y- axes simultaneously, so they
    // offer "shortcuts" back to the center hex that would be missed if movement were restricted
    // along only the x- and y- axes. So prioritize southeast or northwest steps by handling
    // them first.
    let mut extra_steps = 0;

    // Move northeast for as long as it's possible.
    while hex_x > 0 && hex_y < 0 {
        extra_steps += 1;
        hex_x -= 1;
        hex_y += 1;
    }

    // Move southwest for as long as it's possible.
    while hex_x < 0 && hex_y > 0 {
        extra_steps += 1;
        hex_x += 1;
        hex_y -= 1;
    }

    hex_x.abs() + hex_y.abs() + extra_steps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("ne,ne,ne"), Solution::U8(3));
    }
    #[test]
    fn example1_2() {
        assert_eq!(solve_1("ne,ne,sw,sw"), Solution::U8(0));
    }
    #[test]
    fn example1_3() {
        assert_eq!(solve_1("ne,ne,s,s"), Solution::U8(2));
    }
    #[test]
    fn example1_4() {
        assert_eq!(solve_1("se,sw,se,sw,sw"), Solution::U8(3));
    }
}
