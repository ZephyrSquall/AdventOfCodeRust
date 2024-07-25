use crate::solver::{Solution, Solver};
use rustc_hash::FxHashSet;

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 3,
    title: "Perfectly Spherical Houses in a Vacuum",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    #[derive(PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }
    // Hash sets only store unique values, so keeping track of houses visited in a hash set ensures
    // each house is counted once no matter how many times it's visited.
    let mut visited_positions = FxHashSet::default();
    visited_positions.insert(Point { x: 0, y: 0 });
    let mut position = Point { x: 0, y: 0 };

    for direction in input.chars() {
        match direction {
            '^' => {
                position.y -= 1;
            }
            '>' => {
                position.x += 1;
            }
            'v' => {
                position.y += 1;
            }
            '<' => {
                position.x -= 1;
            }
            _ => {
                panic!("character does not represent a direction");
            }
        }
        visited_positions.insert(position);
    }

    Solution::USize(visited_positions.len())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1(">"), Solution::U8(2));
    }
    #[test]
    fn example1_2() {
        assert_eq!(solve_1("^>v<"), Solution::U8(4));
    }
    #[test]
    fn example1_3() {
        assert_eq!(solve_1("^v^v^v^v^v"), Solution::U8(2));
    }
}
