use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2023,
    day: 10,
    title: "Pipe Maze",
    part_solvers: &[solve_1],
};

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug)]
enum PipeShape {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
}
impl PipeShape {
    fn new(character: char) -> PipeShape {
        match character {
            '|' => PipeShape::Vertical,
            '-' => PipeShape::Horizontal,
            'L' => PipeShape::NorthEast,
            'J' => PipeShape::NorthWest,
            '7' => PipeShape::SouthWest,
            'F' => PipeShape::SouthEast,
            '.' => PipeShape::Ground,
            _ => panic!("Invalid pipe character"),
        }
    }

    fn next_direction(self, incoming_direction: Direction) -> Direction {
        // Simplifying this match statement by combining arms that lead to the same result would
        // make it more confusing, as it would prevent identical pipe shapes from being listed next
        // to each other.
        #[allow(clippy::match_same_arms)]
        match (self, incoming_direction) {
            (PipeShape::Vertical, Direction::North) => Direction::North,
            (PipeShape::Vertical, Direction::South) => Direction::South,
            (PipeShape::Horizontal, Direction::East) => Direction::East,
            (PipeShape::Horizontal, Direction::West) => Direction::West,
            (PipeShape::NorthEast, Direction::South) => Direction::East,
            (PipeShape::NorthEast, Direction::West) => Direction::North,
            (PipeShape::NorthWest, Direction::South) => Direction::West,
            (PipeShape::NorthWest, Direction::East) => Direction::North,
            (PipeShape::SouthWest, Direction::North) => Direction::West,
            (PipeShape::SouthWest, Direction::East) => Direction::South,
            (PipeShape::SouthEast, Direction::North) => Direction::East,
            (PipeShape::SouthEast, Direction::West) => Direction::South,
            _ => panic!("Invalid incoming direction"),
        }
    }
    fn connects_north(self) -> bool {
        matches!(
            self,
            PipeShape::Vertical | PipeShape::NorthEast | PipeShape::NorthWest
        )
    }
    fn connects_east(self) -> bool {
        matches!(
            self,
            PipeShape::Horizontal | PipeShape::NorthEast | PipeShape::SouthEast
        )
    }
    fn connects_south(self) -> bool {
        matches!(
            self,
            PipeShape::Vertical | PipeShape::SouthEast | PipeShape::SouthWest
        )
    }
    fn connects_west(self) -> bool {
        matches!(
            self,
            PipeShape::Horizontal | PipeShape::NorthWest | PipeShape::SouthWest
        )
    }
}

struct Grid {
    pipes: Vec<Vec<PipeShape>>,
    starting_x: usize,
    starting_y: usize,
}
impl Grid {
    fn new(input: &str) -> Grid {
        let mut starting_x = 0;
        let mut starting_y = 0;

        let pipes = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            starting_x = x;
                            starting_y = y;
                            // This is a placeholder to be overridden later. Determining the
                            // starting space's shape requires looking at neighboring grid
                            // positions, which can't be done until the grid is fully built.
                            PipeShape::Ground
                        } else {
                            PipeShape::new(c)
                        }
                    })
                    .collect()
            })
            .collect();

        let mut grid = Grid {
            pipes,
            starting_x,
            starting_y,
        };

        // The puzzle description specifies that the starting tile has exactly two neighboring pipes
        // connecting to it, so use this to find the starting tile's shape.
        if grid
            .get_north(starting_x, starting_y)
            .is_some_and(PipeShape::connects_south)
        {
            if grid
                .get_east(starting_x, starting_y)
                .is_some_and(PipeShape::connects_west)
            {
                grid.pipes[starting_y][starting_x] = PipeShape::NorthEast;
            } else if grid
                .get_south(starting_x, starting_y)
                .is_some_and(PipeShape::connects_north)
            {
                grid.pipes[starting_y][starting_x] = PipeShape::Vertical;
            } else if grid
                .get_west(starting_x, starting_y)
                .is_some_and(PipeShape::connects_east)
            {
                grid.pipes[starting_y][starting_x] = PipeShape::NorthWest;
            }
        } else if grid
            .get_east(starting_x, starting_y)
            .is_some_and(PipeShape::connects_west)
        {
            if grid
                .get_south(starting_x, starting_y)
                .is_some_and(PipeShape::connects_north)
            {
                grid.pipes[starting_y][starting_x] = PipeShape::SouthEast;
            } else if grid
                .get_west(starting_x, starting_y)
                .is_some_and(PipeShape::connects_east)
            {
                grid.pipes[starting_y][starting_x] = PipeShape::Horizontal;
            }
        } else {
            grid.pipes[starting_y][starting_x] = PipeShape::SouthWest;
        }

        grid
    }

    fn get(&self, x: usize, y: usize) -> PipeShape {
        self.pipes[y][x]
    }
    fn get_north(&self, x: usize, y: usize) -> Option<PipeShape> {
        y.checked_sub(1).map(|y_sub| self.pipes[y_sub][x])
    }
    fn get_east(&self, x: usize, y: usize) -> Option<PipeShape> {
        self.pipes[y].get(x + 1).copied()
    }
    fn get_south(&self, x: usize, y: usize) -> Option<PipeShape> {
        self.pipes.get(y + 1).map(|grid_line| grid_line[x])
    }
    fn get_west(&self, x: usize, y: usize) -> Option<PipeShape> {
        x.checked_sub(1).map(|x_sub| self.pipes[y][x_sub])
    }
}

fn solve_1(input: &str) -> Solution {
    let grid = Grid::new(input);

    let mut x = grid.starting_x;
    let mut y = grid.starting_y;
    let mut direction = match grid.get(grid.starting_x, grid.starting_y) {
        pipe if pipe.connects_north() => Direction::North,
        pipe if pipe.connects_east() => Direction::East,
        _ => Direction::South,
    };
    let mut step_count = 0;

    // Follow the pipes from the starting position until it is reached again.
    loop {
        match direction {
            Direction::North => y -= 1,
            Direction::East => x += 1,
            Direction::South => y += 1,
            Direction::West => x -= 1,
        }
        step_count += 1;

        if x == grid.starting_x && y == grid.starting_y {
            break;
        }

        direction = grid.get(x, y).next_direction(direction);
    }

    Solution::U32(step_count / 2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            ),
            Solution::U8(4)
        );
    }
    #[test]
    fn example1_2() {
        assert_eq!(
            solve_1(
                "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"
            ),
            Solution::U8(8)
        );
    }
}
