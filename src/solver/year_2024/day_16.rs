use crate::solver::{Solution, AdventOfCode};
use std::cmp::min;

// Temporarily allow dead code, as this solver had to be disabled for now to prevent stack overflows
// when running all solvers. In the future, this solver will be refactored to not recurse as deeply
// or use a different technique so that it can be reenabled.
#[allow(dead_code)]
pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2024,
    day: 16,
    title: "Reindeer Maze",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    #[derive(Clone)]
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

    #[derive(Clone)]
    struct Reindeer {
        x: usize,
        y: usize,
        direction: Direction,
        score: u32,
    }
    impl Reindeer {
        fn advance(&mut self) {
            match self.direction {
                Direction::North => self.y -= 1,
                Direction::East => self.x += 1,
                Direction::South => self.y += 1,
                Direction::West => self.x -= 1,
            }
            self.score += 1;
        }
        fn turn_left(&mut self) {
            self.direction.turn_left();
            self.score += 1000;
        }
        fn turn_right(&mut self) {
            self.direction.turn_right();
            self.score += 1000;
        }
        fn is_facing_wall(&self, maze: &[Vec<Tile>]) -> bool {
            match self.direction {
                Direction::North => maze[self.y - 1][self.x] == Tile::Wall,
                Direction::East => maze[self.y][self.x + 1] == Tile::Wall,
                Direction::South => maze[self.y + 1][self.x] == Tile::Wall,
                Direction::West => maze[self.y][self.x - 1] == Tile::Wall,
            }
        }
        fn is_at_end(&self, maze: &[Vec<Tile>]) -> bool {
            maze[self.y][self.x] == Tile::End
        }
        // Checks if the minimum score reached at the maze in its current position is greater than
        // the current score. If so, sets the minimum value to the score, and returns true.
        // Otherwise, does nothing and returns false.
        fn update_maze_at_position(&self, maze: &mut [Vec<Tile>]) -> bool {
            if let Tile::Empty(minimum_score_in_direction) = &mut maze[self.y][self.x] {
                let minimum_score = match self.direction {
                    Direction::North => &mut minimum_score_in_direction.north,
                    Direction::East => &mut minimum_score_in_direction.east,
                    Direction::South => &mut minimum_score_in_direction.south,
                    Direction::West => &mut minimum_score_in_direction.west,
                };
                if *minimum_score > self.score {
                    *minimum_score = self.score;
                    return true;
                }
            }
            false
        }
    }

    // For each empty tile, to prevent loops when searching the maze for the best path, keep track
    // of the minimum score seen by a reindeer in each facing direction.
    #[derive(PartialEq)]
    struct MinimumScoreInDirection {
        north: u32,
        east: u32,
        south: u32,
        west: u32,
    }

    #[derive(PartialEq)]
    enum Tile {
        Empty(MinimumScoreInDirection),
        Wall,
        End,
    }

    // Recursively search every possible path to find the one with the minimum score.
    fn find_minimum_score(reindeer: &Reindeer, maze: &mut [Vec<Tile>]) -> u32 {
        // At each step, there are only three actions worth considering: Advancing forward one step,
        // turning to the left, or turning to the right. Any other action is simply a combination of
        // these actions. Split the current reindeer into three new reindeer who each take a
        // different one of these three actions. For each of them, determine if they could possibly
        // reach a new minimum score, and if so, add them to the list of valid reindeer.
        let mut valid_reindeers = Vec::with_capacity(3);

        // Only advance if a wall isn't in the way.
        if !reindeer.is_facing_wall(maze) {
            let mut reindeer_advance = reindeer.clone();
            reindeer_advance.advance();

            // If the reindeer advanced to the end of the maze, return its score. Advancing is the
            // only way to move to a new tile, so this check doesn't need to be performed anywhere
            // else.
            if reindeer_advance.is_at_end(maze) {
                return reindeer_advance.score;
            }

            // Check the minimum score of the maze ever reached at that position in the reindeer's
            // current facing direction. If this reindeer's score is greater than or equal to this
            // minimum score, then it has already been explored by a reindeer doing just as well or
            // better than this reindeer, so there's no point to checking it again.
            if reindeer_advance.update_maze_at_position(maze) {
                valid_reindeers.push(reindeer_advance);
            }
        }

        // For the reindeer that turn, it's only worth considering their movement if they don't turn
        // to face a wall. If they do face a wall, they can't advance from that position, and
        // turning again is pointless as they'd either do a 180 and start retracing their steps or
        // turn the way they just faced with an additional 2000 score. Either way, a reindeer that
        // turns to face a wall can't possibly be a minimum-scoring reindeer. Also check the minimum
        // score at that position in the new direction for the same reason as the advancing
        // reindeer.
        let mut reindeer_turn_left = reindeer.clone();
        reindeer_turn_left.turn_left();
        if !reindeer_turn_left.is_facing_wall(maze)
            && reindeer_turn_left.update_maze_at_position(maze)
        {
            valid_reindeers.push(reindeer_turn_left);
        }

        let mut reindeer_turn_right = reindeer.clone();
        reindeer_turn_right.turn_right();
        if !reindeer_turn_right.is_facing_wall(maze)
            && reindeer_turn_right.update_maze_at_position(maze)
        {
            valid_reindeers.push(reindeer_turn_right);
        }

        // For each valid reindeer, get the minimum score that all of them found. Initalize the
        // minimum score to the maximum possible u32 value, so that if all the reindeer fail to make
        // a new minimum score, or there are simply no valid reindeer, the score returned from this
        // function can't possibly be a new minimum score.
        let mut minimum_score = u32::MAX;
        for valid_reindeer in valid_reindeers {
            minimum_score = min(minimum_score, find_minimum_score(&valid_reindeer, maze));
        }

        minimum_score
    }

    // Read the maze from the puzzle input. Set every empty tile's minimum score in each direction
    // to be the maximum u32 value, so the first actual score placed in it will always be lower than
    // the initial value. However, don't do this for the east direction of the start tile, as this
    // is the reindeer's initial position, whose minimum score in each direction is known to just be
    // the score of rotating to face that direction (0 for east, 1000 for north and south, and 2000
    // for west, as reindeer start facing east).
    let mut maze = Vec::new();
    let mut reindeer_start_east = Reindeer {
        x: 0,
        y: 0,
        direction: Direction::East,
        score: 0,
    };
    for (y, line) in input.lines().enumerate() {
        let mut maze_line = Vec::new();
        for (x, tile) in line.chars().enumerate() {
            match tile {
                '.' => maze_line.push(Tile::Empty(MinimumScoreInDirection {
                    north: u32::MAX,
                    east: u32::MAX,
                    south: u32::MAX,
                    west: u32::MAX,
                })),
                '#' => maze_line.push(Tile::Wall),
                'S' => {
                    maze_line.push(Tile::Empty(MinimumScoreInDirection {
                        north: 1000,
                        east: 0,
                        south: 1000,
                        west: 2000,
                    }));
                    reindeer_start_east.x = x;
                    reindeer_start_east.y = y;
                }
                'E' => maze_line.push(Tile::End),
                _ => panic!("Encountered unsupported tile"),
            }
        }
        maze.push(maze_line);
    }

    // The starting position is the only position from which it may be valid to turn to the side
    // twice in a row. Normally turning to face a wall isn't considered a valid move because turning
    // twice can't be optimal anywhere else. So manually get a list of reindeer facing all four
    // directions to make sure turning twice at the start is considered.
    let mut reindeer_start_north = reindeer_start_east.clone();
    reindeer_start_north.turn_left();
    let mut reindeer_start_west = reindeer_start_north.clone();
    reindeer_start_west.turn_left();
    // The lowest score possible for a reindeer facing south comes from the starting reindeer
    // turning right, not turning left three times.
    let mut reindeer_start_south = reindeer_start_east.clone();
    reindeer_start_south.turn_right();

    let starting_reindeers = vec![
        reindeer_start_north,
        reindeer_start_east,
        reindeer_start_south,
        reindeer_start_west,
    ];

    // For each starting reindeer, get the minimum score, and find the minimum of those four scores.
    let mut minimum_score = u32::MAX;
    for starting_reindeer in starting_reindeers {
        minimum_score = min(
            minimum_score,
            find_minimum_score(&starting_reindeer, &mut maze),
        );
    }
    Solution::U32(minimum_score)
}

fn solve_2(input: &str) -> Solution {
    #[derive(Clone)]
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

    #[derive(Clone)]
    struct Reindeer {
        x: usize,
        y: usize,
        direction: Direction,
        score: u32,
    }
    impl Reindeer {
        fn advance(&mut self) {
            match self.direction {
                Direction::North => self.y -= 1,
                Direction::East => self.x += 1,
                Direction::South => self.y += 1,
                Direction::West => self.x -= 1,
            }
            self.score += 1;
        }
        fn turn_left(&mut self) {
            self.direction.turn_left();
            self.score += 1000;
        }
        fn turn_right(&mut self) {
            self.direction.turn_right();
            self.score += 1000;
        }
        fn is_facing_wall(&self, maze: &[Vec<Tile>]) -> bool {
            match self.direction {
                Direction::North => maze[self.y - 1][self.x] == Tile::Wall,
                Direction::East => maze[self.y][self.x + 1] == Tile::Wall,
                Direction::South => maze[self.y + 1][self.x] == Tile::Wall,
                Direction::West => maze[self.y][self.x - 1] == Tile::Wall,
            }
        }
        fn is_at_end(&self, maze: &[Vec<Tile>]) -> bool {
            maze[self.y][self.x] == Tile::End
        }
        // Note that unlike in part 1, it's important to let the reindeer keep exploring if the
        // minimum_score is equal to the reindeer's current score, not just strictly greater then.
        // This is to ensure all minimum-scoring paths are found instead of just one.
        fn update_maze_at_position(&self, maze: &mut [Vec<Tile>]) -> bool {
            if let Tile::Empty(minimum_scores) = &mut maze[self.y][self.x] {
                let minimum_score = match self.direction {
                    Direction::North => &mut minimum_scores.north,
                    Direction::East => &mut minimum_scores.east,
                    Direction::South => &mut minimum_scores.south,
                    Direction::West => &mut minimum_scores.west,
                };
                if *minimum_score >= self.score {
                    *minimum_score = self.score;
                    return true;
                }
            }
            false
        }
    }

    // In addition to keeping track of the minimum scores of reindeers in each direction, track the
    // smallest final score of reindeers who took this path and reached the end, so that all tiles
    // on at least one minimum-scoring path can be found after searching all minimum-scoring paths.
    #[derive(PartialEq)]
    struct MinimumScores {
        north: u32,
        east: u32,
        south: u32,
        west: u32,
        at_end: u32,
    }

    #[derive(PartialEq)]
    enum Tile {
        Empty(MinimumScores),
        Wall,
        End,
    }

    fn find_minimum_score(reindeer: &Reindeer, maze: &mut [Vec<Tile>]) -> u32 {
        let mut valid_reindeers = Vec::with_capacity(3);

        if !reindeer.is_facing_wall(maze) {
            let mut reindeer_advance = reindeer.clone();
            reindeer_advance.advance();

            if reindeer_advance.is_at_end(maze) {
                // Before returning the new minimum score, update this tile with the new at-end
                // minimum score, as otherwise it will be missed (the end will also be missed, but
                // there's exactly one end tile so this can be accounted for by adding 1 to the
                // final total).
                let minimum_score_at_end = match &mut maze[reindeer.y][reindeer.x] {
                    Tile::Empty(minimum_scores) => &mut minimum_scores.at_end,
                    _ => panic!("This tile should be empty"),
                };
                *minimum_score_at_end = min(reindeer_advance.score, *minimum_score_at_end);

                return reindeer_advance.score;
            }

            if reindeer_advance.update_maze_at_position(maze) {
                valid_reindeers.push(reindeer_advance);
            }
        }

        let mut reindeer_turn_left = reindeer.clone();
        reindeer_turn_left.turn_left();
        if !reindeer_turn_left.is_facing_wall(maze)
            && reindeer_turn_left.update_maze_at_position(maze)
        {
            valid_reindeers.push(reindeer_turn_left);
        }

        let mut reindeer_turn_right = reindeer.clone();
        reindeer_turn_right.turn_right();
        if !reindeer_turn_right.is_facing_wall(maze)
            && reindeer_turn_right.update_maze_at_position(maze)
        {
            valid_reindeers.push(reindeer_turn_right);
        }

        let mut minimum_score = u32::MAX;
        for valid_reindeer in valid_reindeers {
            minimum_score = min(minimum_score, find_minimum_score(&valid_reindeer, maze));
        }

        // Update this tile's at-end minimum score if the minimum score just found is smaller.
        let minimum_score_at_end = match &mut maze[reindeer.y][reindeer.x] {
            Tile::Empty(minimum_scores) => &mut minimum_scores.at_end,
            _ => panic!("This tile should be empty"),
        };
        *minimum_score_at_end = min(minimum_score, *minimum_score_at_end);

        minimum_score
    }

    let mut maze = Vec::new();
    let mut reindeer_start_east = Reindeer {
        x: 0,
        y: 0,
        direction: Direction::East,
        score: 0,
    };
    for (y, line) in input.lines().enumerate() {
        let mut maze_line = Vec::new();
        for (x, tile) in line.chars().enumerate() {
            match tile {
                '.' => maze_line.push(Tile::Empty(MinimumScores {
                    north: u32::MAX,
                    east: u32::MAX,
                    south: u32::MAX,
                    west: u32::MAX,
                    at_end: u32::MAX,
                })),
                '#' => maze_line.push(Tile::Wall),
                'S' => {
                    maze_line.push(Tile::Empty(MinimumScores {
                        north: 1000,
                        east: 0,
                        south: 1000,
                        west: 2000,
                        at_end: u32::MAX,
                    }));
                    reindeer_start_east.x = x;
                    reindeer_start_east.y = y;
                }
                'E' => maze_line.push(Tile::End),
                _ => panic!("Encountered unsupported tile"),
            }
        }
        maze.push(maze_line);
    }

    let mut reindeer_start_north = reindeer_start_east.clone();
    reindeer_start_north.turn_left();
    let mut reindeer_start_west = reindeer_start_north.clone();
    reindeer_start_west.turn_left();
    let mut reindeer_start_south = reindeer_start_east.clone();
    reindeer_start_south.turn_right();

    let starting_reindeers = vec![
        reindeer_start_north,
        reindeer_start_east,
        reindeer_start_south,
        reindeer_start_west,
    ];

    let mut minimum_score = u32::MAX;
    for starting_reindeer in starting_reindeers {
        minimum_score = min(
            minimum_score,
            find_minimum_score(&starting_reindeer, &mut maze),
        );
    }

    // With this minimum score, check every tile and count the tiles whose at-end minimum score
    // matches it. Start the count at 1 to account for the end tile being intentionally skipped.
    let mut tiles_with_correct_at_end_score = 1;
    for maze_line in maze {
        for tile in maze_line {
            if let Tile::Empty(tile_minimum_scores) = tile {
                if tile_minimum_scores.at_end == minimum_score {
                    tiles_with_correct_at_end_score += 1;
                }
            }
        }
    }
    Solution::U32(tiles_with_correct_at_end_score)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            ),
            Solution::U16(7036)
        );
    }
    #[test]
    fn example1_2() {
        assert_eq!(
            solve_1(
                "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
            ),
            Solution::U16(11048)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            ),
            Solution::U8(45)
        );
    }
    #[test]
    fn example2_2() {
        assert_eq!(
            solve_2(
                "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
            ),
            Solution::U8(64)
        );
    }
}
