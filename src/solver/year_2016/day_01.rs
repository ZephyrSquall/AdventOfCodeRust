use crate::solver::{Solution, Solver};
use std::mem::swap;

pub const SOLVER: Solver = Solver {
    year: 2016,
    day: 1,
    title: "No Time for a Taxicab",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
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

fn solve_2(input: &str) -> Solution {
    // A line that is perfectly horizontal or vertical is fully-specified by just three points.
    struct Line {
        // start and end refer to the y-position for horizontal lines and x-position for vertical
        // lines.
        start: i32,
        end: i32,
        // along specifies the x-position that a horizontal line is on, or the y-position that a
        // vertical line is on.
        along: i32,
        // e.g. a horizontal line starts at (along, start) and ends at (along, end), and a vertical
        // line starts at (start, along) and ends at (end, along).
    }
    impl Line {
        fn intersects(&self, other: &Line) -> Option<Position> {
            // It is assumed that one line is vertical and the other is horizontal, since this
            // method is only called in scenarios where that's the case. The following if statement
            // is created by considering that if the lines do intersect and self is the horizontal
            // line, then the situation looks like the following diagram, which gives a
            // self.start <= other.along <= self.end ordering for the x-positions and a
            // other.start <= self.along <= other.end ordering for the y-positions.

            //                 (other.along, other.start)
            //                              o
            //                              |
            // (self.start, self.along) o---+---o (self.end, self.along)
            //                              |
            //                              o
            //                  (other,along, other.end)
            //

            // This situation is symmetric, so if self is actually the vertical line, this logic
            // still gives the correct answer. The returned position will have its x and y swapped
            // around in this case, but that's also fine since the return value is only used to
            // compute the taxicab distance, which is unchanged by swapping its operands.
            if self.start <= other.along
                && other.along <= self.end
                && other.start <= self.along
                && self.along <= other.end
            {
                return Some(Position {
                    x: self.along,
                    y: other.along,
                });
            }
            None
        }
    }

    let mut horizontal_lines = Vec::new();
    let mut vertical_lines = Vec::new();

    let mut position = Position { x: 0, y: 0 };
    let mut last_position = Position { x: 0, y: 0 };
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
        last_position.x = position.x;
        last_position.y = position.y;
        position.walk(distance, &direction);

        if direction.is_horizontal() {
            let mut start = last_position.x;
            let mut end = position.x;
            let along = position.y;
            // Ensure start is the smaller number, or else the line's intersect method will fail to
            // work.
            if start > end {
                swap(&mut start, &mut end);
            }
            let horizontal_line = Line { start, end, along };

            // Check if this line intersects with any vertical lines. If so, calculate the taxicab
            // distance and return it. Otherwise, add this line to the list of horizontal lines.
            // Skip checking intersections with the last vertical line (by reversing the iterator
            // and skipping the first element) because each line always overlaps with the
            // immediately-preceding line at the corner and can't intersect it any other way.
            for vertical_line in vertical_lines.iter().rev().skip(1) {
                if let Some(intersection) = horizontal_line.intersects(vertical_line) {
                    return Solution::I32(intersection.x.abs() + intersection.y.abs());
                }
            }
            horizontal_lines.push(horizontal_line);
        } else {
            // Repeats the same logic as above, but with horizontal lines and vertical lines
            // swapping roles.
            let mut start = last_position.y;
            let mut end = position.y;
            let along = position.x;
            if start > end {
                swap(&mut start, &mut end);
            }
            let vertical_line = Line { start, end, along };

            for horizontal_line in horizontal_lines.iter().rev().skip(1) {
                if let Some(intersection) = vertical_line.intersects(horizontal_line) {
                    return Solution::I32(intersection.x.abs() + intersection.y.abs());
                }
            }
            vertical_lines.push(vertical_line);
        }
    }

    panic!("Should have found an intersection");
}

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
    fn is_horizontal(&self) -> bool {
        match self {
            Direction::North | Direction::South => false,
            Direction::East | Direction::West => true,
        }
    }
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

    #[test]
    fn example2_1() {
        assert_eq!(solve_2("R8, R4, R4, R8"), Solution::U8(4));
    }
}
