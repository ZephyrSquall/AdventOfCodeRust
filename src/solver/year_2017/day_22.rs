use crate::solver::{Solution, AdventOfCode};
use std::{collections::VecDeque, iter};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2017,
    day: 22,
    title: "Sporifica Virus",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    // The following set of four functions extend the grid as needed so it represents an infinite
    // grid.
    fn extend_up(grid: &mut VecDeque<VecDeque<bool>>) {
        grid.push_front(iter::repeat(false).take(grid[0].len()).collect());
    }
    fn extend_down(grid: &mut VecDeque<VecDeque<bool>>) {
        grid.push_back(iter::repeat(false).take(grid[0].len()).collect());
    }
    fn extend_right(grid: &mut VecDeque<VecDeque<bool>>) {
        for grid_line in grid {
            grid_line.push_back(false);
        }
    }
    fn extend_left(grid: &mut VecDeque<VecDeque<bool>>) {
        for grid_line in grid {
            grid_line.push_front(false);
        }
    }
    let mut grid = VecDeque::new();

    // Convert the input to a 2D array of bools (VecDeque is used because being able to push to the
    // front and back makes it efficient to extend the grid in any direction).
    for line in input.lines() {
        let mut grid_line = VecDeque::new();
        for node in line.chars() {
            grid_line.push_back(node == '#');
        }
        grid.push_back(grid_line);
    }

    // Starting state of the virus.
    let mut x = (grid[0].len() - 1) / 2;
    let mut y = (grid.len() - 1) / 2;
    let mut direction = Direction::Up;
    let mut infections = 0;

    for _ in 0..10000 {
        // Use the current node to determine which way to turn.
        if grid[y][x] {
            direction.turn_right();
        } else {
            direction.turn_left();
        }

        // Flip the current node's state. If the node became infected, count the infection.
        grid[y][x] = !grid[y][x];
        if grid[y][x] {
            infections += 1;
        }

        match direction {
            Direction::Up => {
                // If the grid is extended upwards (or leftwards), the index of all nodes is shifted
                // 1 greater, so y (or x) should remain unchanged.
                if y == 0 {
                    extend_up(&mut grid);
                } else {
                    y -= 1;
                }
            }
            Direction::Right => {
                // Extending the grid rightwards (or downwards) does not shift the index, so 1 needs
                // to be added to x (or y) regardless of whether the grid was extended.
                if x == grid[0].len() - 1 {
                    extend_right(&mut grid);
                }
                x += 1;
            }
            Direction::Down => {
                if y == grid.len() - 1 {
                    extend_down(&mut grid);
                }
                y += 1;
            }
            Direction::Left => {
                if x == 0 {
                    extend_left(&mut grid);
                } else {
                    x -= 1;
                }
            }
        }
    }

    Solution::U16(infections)
}

fn solve_2(input: &str) -> Solution {
    #[derive(Clone, Copy, PartialEq)]
    enum Node {
        Clean,
        Weakened,
        Infected,
        Flagged,
    }
    impl Node {
        fn advance(&mut self) {
            {
                match self {
                    Node::Clean => {
                        *self = Node::Weakened;
                    }
                    Node::Weakened => {
                        *self = Node::Infected;
                    }
                    Node::Infected => {
                        *self = Node::Flagged;
                    }
                    Node::Flagged => {
                        *self = Node::Clean;
                    }
                }
            }
        }
    }

    // The following set of four functions extend the grid as needed so it represents an infinite
    // grid.
    fn extend_up(grid: &mut VecDeque<VecDeque<Node>>) {
        grid.push_front(iter::repeat(Node::Clean).take(grid[0].len()).collect());
    }
    fn extend_down(grid: &mut VecDeque<VecDeque<Node>>) {
        grid.push_back(iter::repeat(Node::Clean).take(grid[0].len()).collect());
    }
    fn extend_right(grid: &mut VecDeque<VecDeque<Node>>) {
        for grid_line in grid {
            grid_line.push_back(Node::Clean);
        }
    }
    fn extend_left(grid: &mut VecDeque<VecDeque<Node>>) {
        for grid_line in grid {
            grid_line.push_front(Node::Clean);
        }
    }
    let mut grid = VecDeque::new();

    // Convert the input to a 2D array of Nodes (VecDeque is used because being able to push to the
    // front and back makes it efficient to extend the grid in any direction).
    for line in input.lines() {
        let mut grid_line = VecDeque::new();
        for node in line.chars() {
            if node == '#' {
                grid_line.push_back(Node::Infected);
            } else {
                grid_line.push_back(Node::Clean);
            }
        }
        grid.push_back(grid_line);
    }

    // Starting state of the virus.
    let mut x = (grid[0].len() - 1) / 2;
    let mut y = (grid.len() - 1) / 2;
    let mut direction = Direction::Up;
    let mut infections = 0;

    for _ in 0..10_000_000 {
        // Use the current node to determine which way to turn.
        match grid[y][x] {
            Node::Clean => {
                direction.turn_left();
            }
            Node::Weakened => {}
            Node::Infected => {
                direction.turn_right();
            }
            Node::Flagged => {
                direction.reverse();
            }
        }

        // Advance the current node's state. If the node became infected, count the infection.
        grid[y][x].advance();
        if grid[y][x] == Node::Infected {
            infections += 1;
        }

        match direction {
            Direction::Up => {
                // If the grid is extended upwards (or leftwards), the index of all nodes is shifted
                // 1 greater, so y (or x) should remain unchanged.
                if y == 0 {
                    extend_up(&mut grid);
                } else {
                    y -= 1;
                }
            }
            Direction::Right => {
                // Extending the grid rightwards (or downwards) does not shift the index, so 1 needs
                // to be added to x (or y) regardless of whether the grid was extended.
                if x == grid[0].len() - 1 {
                    extend_right(&mut grid);
                }
                x += 1;
            }
            Direction::Down => {
                if y == grid.len() - 1 {
                    extend_down(&mut grid);
                }
                y += 1;
            }
            Direction::Left => {
                if x == 0 {
                    extend_left(&mut grid);
                } else {
                    x -= 1;
                }
            }
        }
    }

    Solution::U32(infections)
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn turn_left(&mut self) {
        match self {
            Direction::Up => {
                *self = Direction::Left;
            }
            Direction::Right => {
                *self = Direction::Up;
            }
            Direction::Down => {
                *self = Direction::Right;
            }
            Direction::Left => {
                *self = Direction::Down;
            }
        }
    }
    fn turn_right(&mut self) {
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
    fn reverse(&mut self) {
        match self {
            Direction::Up => {
                *self = Direction::Down;
            }
            Direction::Right => {
                *self = Direction::Left;
            }
            Direction::Down => {
                *self = Direction::Up;
            }
            Direction::Left => {
                *self = Direction::Right;
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
                "\
..#
#..
..."
            ),
            Solution::U16(5587)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
..#
#..
..."
            ),
            Solution::U32(2_511_944)
        );
    }
}
