use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2016,
    day: 1,
    title: "No Time for a Taxicab",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    struct Position {
        x: i32,
        y: i32,
    }
    impl Position {
        fn walk(&mut self, distance: i32, direction: &Direction) {
            match direction {
                Direction::North => self.y -= distance,
                Direction::East => self.x += distance,
                Direction::South => self.y += distance,
                Direction::West => self.x -= distance,
            }
        }
    }

    enum Direction {
        North,
        East,
        South,
        West,
    }
    impl Direction {
        fn turn_left(&mut self) {
            match self {
                Direction::North => *self = Direction::West,
                Direction::East => *self = Direction::North,
                Direction::South => *self = Direction::East,
                Direction::West => *self = Direction::South,
            }
        }
        fn turn_right(&mut self) {
            match self {
                Direction::North => *self = Direction::East,
                Direction::East => *self = Direction::South,
                Direction::South => *self = Direction::West,
                Direction::West => *self = Direction::North,
            }
        }
    }

    let mut position = Position { x: 0, y: 0 };
    let mut direction = Direction::North;

    for instruction in input.split(", ") {
        let mut char_iter = instruction.chars();
        // Assume that if the first character isn't 'L', then it's 'R'.
        if char_iter
            .next()
            .expect("Instruction should have first character")
            == 'L'
        {
            direction.turn_left();
        } else {
            direction.turn_right();
        }

        let distance = char_iter
            .as_str()
            .parse()
            .expect("Rest of instruction should be a number");

        position.walk(distance, &direction);
    }

    Solution::I32(position.x.abs() + position.y.abs())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("R2, L3"), Solution::U8(5));
    }
    #[test]
    fn example1_2() {
        assert_eq!(solve_1("R2, R2, R2"), Solution::U8(2));
    }
    #[test]
    fn example1_3() {
        assert_eq!(solve_1("R5, L5, R5, R3"), Solution::U8(12));
    }
}
