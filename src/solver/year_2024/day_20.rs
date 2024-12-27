use crate::solver::{AdventOfCode, Solution};
use rustc_hash::FxHashMap;
use std::cmp::min;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2024,
    day: 20,
    title: "Race Condition",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let (maze, start_position, end_position, x_len, y_len) =
        get_maze_and_start_and_end_and_lens(input);
    let shortest_path_steps = get_shortest_path_steps(&maze, &start_position, &end_position);

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

fn solve_2(input: &str) -> Solution {
    let (maze, start_position, end_position, x_len, y_len) =
        get_maze_and_start_and_end_and_lens(input);
    let shortest_path_steps = get_shortest_path_steps(&maze, &start_position, &end_position);

    // Using the following code, it was determined that my puzzle input has no branching paths
    // whatsoever. This is a stronger condition than there being only a single best path, as there's
    // no need to consider other paths whatsoever (it otherwise could have been the case that moving
    // into a dead end would be optimal to prepare for a cheat, which would have been significantly
    // harder to account for). As this code is not needed for the solution itself, it is commented
    // out but left here for reference:

    // for (y, maze_line) in maze.iter().enumerate() {
    //     for (x, tile) in maze_line.iter().enumerate() {
    //         if *tile {
    //             let mut count = 0;
    //             if maze[y - 1][x] {
    //                 count += 1;
    //             }
    //             if maze[y][x + 1] {
    //                 count += 1;
    //             }
    //             if maze[y + 1][x] {
    //                 count += 1;
    //             }
    //             if maze[y][x - 1] {
    //                 count += 1;
    //             }
    //             if count > 2 {
    //                 println!("BRANCHING PATH DETECTED");
    //             }
    //         }
    //     }
    // }

    // After this, I realized that the puzzle description actually specified there's only one path,
    // not that there's only one shortest path, but it's nice to have verified this myself!

    // Create a "diamond" of possible cheat destinations around every tile on the path. Start by
    // moving up to 20 spaces along the y axis in either direction, stopping if the bounds of the
    // maze are reached. Then move up to 20 - y_offset spaces along the x axis in either direction,
    // where the y_offset is how many spaces had been moved along the y axis, again stopping if the
    // bounds of the maze are reached. With this strategy, every tile within 20 spaces of the
    // current tile is iterated over, which means every possible cheating destination is found. For
    // each tile found this way, check if its picoseconds is greater or equal to the current
    // picoseconds plus the travel distance plus 100. If it is, then cheating to move from the
    // current tile to the other tile saves at least 100 picoseconds, so count this cheat.
    let mut cheats_saving_100_picoseconds = 0;
    for (tile, picoseconds) in &shortest_path_steps {
        let y_min = tile.y.saturating_sub(20);
        let y_max = min(y_len - 1, tile.y + 20);

        for y in y_min..=y_max {
            let y_offset = y.abs_diff(tile.y);
            let x_max_offset = y_offset.abs_diff(20);

            let x_min = tile.x.saturating_sub(x_max_offset);
            let x_max = min(x_len - 1, tile.x + x_max_offset);

            for x in x_min..=x_max {
                let x_offset = x.abs_diff(tile.x);
                let travel_distance = x_offset + y_offset;

                // A travel distance less than 1 cannot possibly skip any walls.
                if travel_distance > 1 {
                    if let Some(other_picoseconds) = shortest_path_steps.get(&Position { x, y }) {
                        if *other_picoseconds >= picoseconds + travel_distance + 100 {
                            cheats_saving_100_picoseconds += 1;
                        }
                    }
                }
            }
        }
    }

    Solution::U32(cheats_saving_100_picoseconds)
}

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

// Read the maze from the input. Return the maze, the start position, the end position, the x length
// of the maze, and the y length of the maze, in that order. The maze is a vector of vector of
// bools, where true represents empty space and false represents a wall.
fn get_maze_and_start_and_end_and_lens(
    input: &str,
) -> (Vec<Vec<bool>>, Position, Position, usize, usize) {
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

    (maze, start_position, end_position, x_len, y_len)
}

// Use the A* algorithm to get a map that contains only positions on the shortest path, and the
// picoseconds elapsed (how many steps taken so far) at each position. This is the only path that
// needs to be considered, as the puzzle description states there is only one path from the start to
// the end.
fn get_shortest_path_steps(
    maze: &[Vec<bool>],
    start_position: &Position,
    end_position: &Position,
) -> FxHashMap<Position, usize> {
    let mut shortest_path_steps = FxHashMap::default();

    let mut open_set = vec![start_position.clone()];
    let mut came_from: FxHashMap<Position, Position> = FxHashMap::default();
    let mut g_score = FxHashMap::default();
    g_score.insert(start_position.clone(), 0);
    let mut f_score = FxHashMap::default();
    f_score.insert(
        start_position.clone(),
        start_position.distance_from(end_position),
    );

    while let Some(current) = open_set.pop() {
        if current == *end_position {
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
                    tentative_g + neighbour.distance_from(end_position),
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

    shortest_path_steps
}

// The examples provided are too different in their format (showing exact picoseconds saved by
// specific cheats instead of how many cheats in total save a certain threshold of picoseconds) to
// be worth testing.
