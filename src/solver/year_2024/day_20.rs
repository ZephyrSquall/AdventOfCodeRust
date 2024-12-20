use crate::solver::{Solution, Solver};
use rustc_hash::FxHashMap;

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 20,
    title: "Race Condition",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    #[derive(PartialEq, Eq, Hash, Clone)]
    struct Position {
        x: usize,
        y: usize,
    }
    impl Position {
        fn distance_from(&self, other: &Position) -> usize {
            self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
        }
    }

    // Read the maze. true represents empty space and false represents a wall.
    let mut maze = Vec::new();
    let mut start_position = Position { x: 0, y: 0 };
    let mut end_position = Position { x: 0, y: 0 };
    for (y, line) in input.lines().enumerate() {
        let mut maze_line = Vec::new();
        for (x, tile) in line.chars().enumerate() {
            let tile = match tile {
                '#' => false,
                '.' => true,
                'S' => {
                    start_position.x = x;
                    start_position.y = y;
                    true
                }
                'E' => {
                    end_position.x = x;
                    end_position.y = y;
                    true
                }
                _ => panic!("Unsupported tile character"),
            };
            maze_line.push(tile);
        }
        maze.push(maze_line);
    }

    let x_len = maze[0].len();
    let y_len = maze.len();

    // Use the A* algorithm to get a map that contains only positions on the shortest path, and the
    // picoseconds elapsed (how many steps taken so far) at each position. This is the only path
    // that needs to be considered, as the puzzle description states there is only one path from the
    // start to the end.
    let mut shortest_path_steps = FxHashMap::default();

    let mut open_set = vec![start_position.clone()];
    let mut came_from: FxHashMap<Position, Position> = FxHashMap::default();
    let mut g_score = FxHashMap::default();
    g_score.insert(start_position.clone(), 0);
    let mut f_score = FxHashMap::default();
    f_score.insert(
        start_position.clone(),
        start_position.distance_from(&end_position),
    );

    while let Some(current) = open_set.pop() {
        if current == end_position {
            shortest_path_steps.insert(
                current.clone(),
                *g_score
                    .get(&current)
                    .expect("Every tile on the shortest path should have a g score"),
            );
            let mut last_position = current.clone();
            while let Some(next_position) = came_from.get(&last_position) {
                shortest_path_steps.insert(
                    next_position.clone(),
                    *g_score
                        .get(next_position)
                        .expect("Every tile on the shortest path should have a g score"),
                );
                last_position = next_position.clone();
            }
        }

        let mut neighbours = Vec::with_capacity(4);
        // Bounds checks are not required for neighbours as the maze is surrounded by a layer of
        // walls.
        if maze[current.y - 1][current.x] {
            neighbours.push(Position {
                x: current.x,
                y: current.y - 1,
            });
        }
        if maze[current.y][current.x + 1] {
            neighbours.push(Position {
                x: current.x + 1,
                y: current.y,
            });
        }
        if maze[current.y + 1][current.x] {
            neighbours.push(Position {
                x: current.x,
                y: current.y + 1,
            });
        }
        if maze[current.y][current.x - 1] {
            neighbours.push(Position {
                x: current.x - 1,
                y: current.y,
            });
        }

        let tentative_g = *g_score.entry(current.clone()).or_insert(usize::MAX) + 1;
        for neighbour in neighbours {
            let neighbour_g = g_score.entry(neighbour.clone()).or_insert(usize::MAX);
            if tentative_g < *neighbour_g {
                came_from.insert(neighbour.clone(), current.clone());
                *neighbour_g = tentative_g;
                f_score.insert(
                    neighbour.clone(),
                    tentative_g + neighbour.distance_from(&end_position),
                );
                if !open_set.contains(&neighbour) {
                    open_set.push(neighbour);
                }
            }
        }

        // Sort the open set so the position with the lowest f score is moved to the end, to be
        // popped next.
        open_set.sort_unstable_by(|a, b| f_score.get(a).cmp(&f_score.get(b)).reverse());
    }

    // For each tile on the shortest path, check in all four directions for a wall. In each
    // direction where there's a wall, check if there's an empty tile on the other side who also
    // belongs to the shortest path and whose picoseconds is at least 102 greater than the current
    // picosecond (so it saves at least 100 steps after factoring in the 2 extra steps taken to
    // travel through the wall). If so, count it.
    let mut cheats_saving_100_picoseconds = 0;
    for (tile, picoseconds) in &shortest_path_steps {
        // Check up.
        if tile.y >= 2 && !maze[tile.y - 1][tile.x] {
            if let Some(other_picoseconds) = shortest_path_steps.get(&Position {
                x: tile.x,
                y: tile.y - 2,
            }) {
                if *other_picoseconds >= *picoseconds + 102 {
                    cheats_saving_100_picoseconds += 1;
                }
            }
        }

        // Check right.
        if tile.x + 2 < x_len && !maze[tile.y][tile.x + 1] {
            if let Some(other_picoseconds) = shortest_path_steps.get(&Position {
                x: tile.x + 2,
                y: tile.y,
            }) {
                if *other_picoseconds >= *picoseconds + 102 {
                    cheats_saving_100_picoseconds += 1;
                }
            }
        }

        // Check down.
        if tile.y + 2 < y_len && !maze[tile.y + 1][tile.x] {
            if let Some(other_picoseconds) = shortest_path_steps.get(&Position {
                x: tile.x,
                y: tile.y + 2,
            }) {
                if *other_picoseconds >= *picoseconds + 102 {
                    cheats_saving_100_picoseconds += 1;
                }
            }
        }

        // Check left.
        if tile.x >= 2 && !maze[tile.y][tile.x - 1] {
            if let Some(other_picoseconds) = shortest_path_steps.get(&Position {
                x: tile.x - 2,
                y: tile.y,
            }) {
                if *other_picoseconds >= *picoseconds + 102 {
                    cheats_saving_100_picoseconds += 1;
                }
            }
        }
    }

    Solution::U32(cheats_saving_100_picoseconds)
}

// The examples provided are too different in their format (showing exact picoseconds saved by
// specific cheats instead of how many cheats in total save a certain threshold of picoseconds) to
// be worth testing.
